mod ap;
mod lbcf;
mod tuning;

use crate::{PCM, Stereo};
use ap::AllPass;
use lbcf::Comb;

/// Implementation of the "freeverb" algorithm.
///
/// Each of the internal combs and allpass filters are limited to a maximum of `N` samples.
///
/// Freeverb is a Schroeder reverberator and was written by "Jezar at Dreampoint". It uses eight parallel Schroeder-Moorer
/// filtered-feedback comb-filters followed by four allpass filters in series for the left and
/// right channels. The right channels are slightly deturned to produce a stereo effect.
///
/// A complete analysis of the algorithm and Comb/All Pass blocks can be found [here](https://www.dsprelated.com/freebooks/pasp/Freeverb.html).
pub struct Freeverb<S: PCM, const N: usize> {
    parameters: FreeverbParameters,
    derived: FreeverbDerivedVars,
    combs_l: [Comb<S, N>; tuning::NUM_COMBS],
    combs_r: [Comb<S, N>; tuning::NUM_COMBS],
    allpass_l: [AllPass<S, N>; tuning::NUM_ALLPASS],
    allpass_r: [AllPass<S, N>; tuning::NUM_ALLPASS],
}

/// Mode for the reverb effect.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FreeverbMode {
    /// Normal mode for a live reverb effect.
    Active = 0,
    /// "Freezes" the reverb, allowing for an infinite tail. Will not incorporate newer signals
    /// until unfrozen.
    Frozen = 1,
}

impl From<u16> for FreeverbMode {
    /// Construct a mode from any unsigned integer. If the value exceeds the number of modes, it
    /// will use the value modulo the number of modes.
    fn from(value: u16) -> Self {
        match value & 1 == 0 {
            true => Self::Active,
            false => Self::Frozen,
        }
    }
}

/// Parameters to the freeverb effect.
#[derive(Clone, Copy)]
pub struct FreeverbParameters {
    /// Mode for the reverb (active or frozen).
    pub mode: FreeverbMode,
    /// Size of the room to model reflections. 0.0 = small room to 1.0 = large room.
    pub room_size: f32,
    /// Amount of damping applied to high frequencies over time. 0.0 = no damping, 1.0 = full
    /// damping.
    pub damp: f32,
    /// Mix of the reverb feedback signal to apply. 0.0 = no wet to 1.0 = full wet.
    pub wet: f32,
    /// Mix of the dry input signal to apply. 0.0 = no dry to 1.0 = full dry.
    pub dry: f32,
    /// Spatial spread of the reverb effect. 0.0 = mono to 1.0 = full stereo.
    pub width: f32,
}

struct FreeverbDerivedVars {
    gain: f32,
    wet_l: f32,
    wet_r: f32,
    dry: f32,
    room_size: f32,
    damp: f32,
}

impl<S: PCM, const N: usize> Freeverb<S, N> {
    /// Construct a freeverb effect with the given initial parameters. All float parameters are
    /// checked to be within the range 0.0..=1.0.
    //
    /// ```
    /// use dspkit::effects::{Freeverb, FreeverbParameters, FreeverbMode};
    /// let freeverb = Freeverb::<f32, 1024>::new(FreeverbParameters {
    ///     mode: FreeverbMode::Active,
    ///     room_size: 0.5,
    ///     damp: 0.5,
    ///     wet: 0.7,
    ///     dry: 0.3,
    ///     width: 0.4
    /// });
    /// ```
    pub fn new(parameters: FreeverbParameters) -> Self {
        Self {
            parameters,
            derived: compute_derived_parameters(parameters),
            combs_l: [Comb::const_default(); tuning::NUM_COMBS],
            combs_r: [Comb::const_default(); tuning::NUM_COMBS],
            allpass_l: [AllPass::const_default(); tuning::NUM_ALLPASS],
            allpass_r: [AllPass::const_default(); tuning::NUM_ALLPASS],
        }
    }

    /// Default const constructor, i.e. can be created at compile-time.   
    /// ```
    /// use dspkit::effects::Freeverb;;
    ///
    /// static FREEVERB: Freeverb<f32, 1024> = Freeverb::const_default();
    /// ```
    pub const fn const_default() -> Self {
        let parameters = FreeverbParameters::const_default();
        Self {
            parameters,
            derived: compute_derived_parameters(parameters),
            combs_l: [Comb::const_default(); tuning::NUM_COMBS],
            combs_r: [Comb::const_default(); tuning::NUM_COMBS],
            allpass_l: [AllPass::const_default(); tuning::NUM_ALLPASS],
            allpass_r: [AllPass::const_default(); tuning::NUM_ALLPASS],
        }
    }

    #[inline]
    pub fn prepare(&mut self, sample_rate: usize) {
        self.derived = compute_derived_parameters(self.parameters);

        for (comb, delay_seconds) in self.combs_l.iter_mut().zip(tuning::COMB_SECOND_TUNINGS) {
            comb.set_feedback(self.derived.room_size);
            comb.set_mix(self.derived.damp);
            comb.set_delay(delay_seconds, sample_rate);
        }

        for (comb, delay_seconds) in self.combs_r.iter_mut().zip(tuning::COMB_SECOND_TUNINGS) {
            comb.set_feedback(self.derived.room_size);
            comb.set_mix(self.derived.damp);
            comb.set_delay(delay_seconds + tuning::STEREO_SPREAD_SEC, sample_rate);
        }

        for (allpass, delay_seconds) in self
            .allpass_l
            .iter_mut()
            .zip(tuning::ALLPASS_SECOND_TUNINGS)
        {
            allpass.set_delay(delay_seconds, sample_rate);
        }

        for (allpass, delay_seconds) in self
            .allpass_r
            .iter_mut()
            .zip(tuning::ALLPASS_SECOND_TUNINGS)
        {
            allpass.set_delay(delay_seconds + tuning::STEREO_SPREAD_SEC, sample_rate);
        }
    }

    pub fn tick(&mut self, input: &Stereo<f32>) -> Stereo<f32> {
        let in_l = input[0];
        let in_r = input[1];

        let mut out_l = f32::PCM_EQUILIBRIUM;
        let mut out_r = f32::PCM_EQUILIBRIUM;

        let mono_input = self.derived.gain * 0.5 * (in_l + in_r);

        for comb in self.combs_l.iter_mut() {
            out_l += comb.tick(&mono_input);
        }

        for comb in self.combs_r.iter_mut() {
            out_r += comb.tick(&mono_input);
        }

        for allpass in self.allpass_l.iter_mut() {
            out_l = allpass.tick(&mono_input);
        }

        for allpass in self.allpass_r.iter_mut() {
            out_r = allpass.tick(&mono_input);
        }

        let wet_l = out_l * self.derived.wet_l + out_r * self.derived.wet_r;
        let wet_r = out_l * self.derived.wet_r + out_r * self.derived.wet_l;

        out_l = wet_l + in_l * self.derived.dry;
        out_r = wet_r + in_r * self.derived.dry;

        [out_l, out_r]
    }

    pub fn set_room_size(&mut self, val: f32) {
        self.parameters.room_size = val;
    }

    pub fn set_damp(&mut self, val: f32) {
        self.parameters.damp = val;
    }

    pub fn set_wet(&mut self, val: f32) {
        self.parameters.wet = val;
    }

    pub fn set_dry(&mut self, val: f32) {
        self.parameters.dry = val;
    }

    pub fn set_width(&mut self, val: f32) {
        self.parameters.width = val;
    }

    pub fn set_mode(&mut self, val: FreeverbMode) {
        self.parameters.mode = val;
    }

    /// Reset the freeverb filter by resetting all of the internal filters.
    pub fn reset(&mut self) -> &mut Self {
        for comb in self.combs_l.iter_mut() {
            comb.reset();
        }
        for comb in self.combs_r.iter_mut() {
            comb.reset();
        }
        for allpass in self.allpass_l.iter_mut() {
            allpass.reset();
        }
        for allpass in self.allpass_r.iter_mut() {
            allpass.reset();
        }
        self
    }
}

impl FreeverbParameters {
    pub const fn const_default() -> Self {
        FreeverbParameters {
            mode: FreeverbMode::Active,
            room_size: tuning::INITIAL_ROOM,
            damp: tuning::INITIAL_DAMP,
            wet: tuning::INITIAL_WET,
            dry: tuning::INITIAL_DRY,
            width: tuning::INITIAL_WIDTH,
        }
    }
}

impl Default for FreeverbParameters {
    fn default() -> Self {
        Self::const_default()
    }
}

impl<S: PCM, const N: usize> Default for Freeverb<S, N> {
    fn default() -> Self {
        Self::const_default()
    }
}

/// Compute derived variables used internally by the freeverb algorithm.
const fn compute_derived_parameters(parameters: FreeverbParameters) -> FreeverbDerivedVars {
    match parameters.mode {
        FreeverbMode::Active => FreeverbDerivedVars {
            gain: tuning::FIXED_GAIN,
            wet_l: tuning::SCALE_WET * parameters.wet * (1.0 + parameters.width) * 0.5,
            wet_r: tuning::SCALE_WET * parameters.wet * (1.0 - parameters.width) * 0.5,
            dry: tuning::SCALE_DRY * parameters.dry,
            room_size: parameters.room_size * tuning::SCALE_ROOM + tuning::OFFSET_ROOM,
            damp: parameters.damp * tuning::SCALE_DAMP,
        },
        FreeverbMode::Frozen => FreeverbDerivedVars {
            gain: 0.0,
            wet_l: tuning::SCALE_WET * parameters.wet * (1.0 + parameters.width) * 0.5,
            wet_r: tuning::SCALE_WET * parameters.wet * (1.0 - parameters.width) * 0.5,
            dry: tuning::SCALE_DRY * parameters.dry,
            room_size: 1.0,
            damp: 0.0,
        },
    }
}
