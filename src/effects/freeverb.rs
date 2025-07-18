use crate::components::{CombFilter, SchroederAllPass};
use crate::{PCM, Stereo};

/// Implementation of the "freeverb" algorithm.
///
/// Each of the internal combs and allpass filters are limited to a maximum of `N` samples.
///
/// Freeverb is a Schroeder reverberator and was written by "Jezar at Dreampoint". It uses eight parallel Schroeder-Moorer
/// filtered-feedback comb-filters followed by four allpass filters in series for the left and
/// right channels. The right channels are slightly deturned to produce a stereo effect.
pub struct Freeverb<S: PCM, const N: usize> {
    parameters: FreeverbParameters,
    derived: FreeverbDerivedVars,
    combs_l: [CombFilter<S, N>; tuning::NUM_COMBS],
    combs_r: [CombFilter<S, N>; tuning::NUM_COMBS],
    allpass_l: [SchroederAllPass<S, N>; tuning::NUM_ALLPASS],
    allpass_r: [SchroederAllPass<S, N>; tuning::NUM_ALLPASS],
}

/// Mode for the reverb effect.
#[derive(Clone, Copy)]
pub enum FreeverbMode {
    /// Normal mode for a live reverb effect.
    Active,
    /// "Freezes" the reverb, allowing for an infinite tail. Will not incorporate newer signals
    /// until unfrozen.
    Frozen,
}

impl From<u8> for FreeverbMode {
    fn from(value: u8) -> Self {
        match value % 2 {
            0 => Self::Active,
            1 => Self::Frozen,
            _ => unreachable!(),
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
            combs_l: [CombFilter::const_default(); tuning::NUM_COMBS],
            combs_r: [CombFilter::const_default(); tuning::NUM_COMBS],
            allpass_l: [SchroederAllPass::const_default(); tuning::NUM_ALLPASS],
            allpass_r: [SchroederAllPass::const_default(); tuning::NUM_ALLPASS],
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
            combs_l: [CombFilter::const_default(); tuning::NUM_COMBS],
            combs_r: [CombFilter::const_default(); tuning::NUM_COMBS],
            allpass_l: [SchroederAllPass::const_default(); tuning::NUM_ALLPASS],
            allpass_r: [SchroederAllPass::const_default(); tuning::NUM_ALLPASS],
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
            allpass.set_feedback(tuning::ALLPASS_FEEDBACK);
            allpass.set_delay(delay_seconds, sample_rate);
        }

        for (allpass, delay_seconds) in self
            .allpass_r
            .iter_mut()
            .zip(tuning::ALLPASS_SECOND_TUNINGS)
        {
            allpass.set_feedback(tuning::ALLPASS_FEEDBACK);
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

    #[inline]
    pub fn set_room_size(&mut self, val: f32) {
        self.parameters.room_size = val;
    }

    #[inline]
    pub fn set_damp(&mut self, val: f32) {
        self.parameters.damp = val;
    }

    #[inline]
    pub fn set_wet(&mut self, val: f32) {
        self.parameters.wet = val;
    }

    #[inline]
    pub fn set_dry(&mut self, val: f32) {
        self.parameters.dry = val;
    }

    #[inline]
    pub fn set_width(&mut self, val: f32) {
        self.parameters.width = val;
    }

    #[inline]
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
            wet_l: parameters.wet * (1.0 + parameters.width) * 0.5,
            wet_r: parameters.wet * (1.0 - parameters.width) * 0.5,
            dry: parameters.dry,
            room_size: parameters.room_size * tuning::SCALE_ROOM + tuning::OFFSET_ROOM,
            damp: parameters.damp * tuning::SCALE_DAMP,
        },
        FreeverbMode::Frozen => FreeverbDerivedVars {
            gain: 0.0,
            wet_l: parameters.wet * (1.0 + parameters.width) * 0.5,
            wet_r: parameters.wet * (1.0 - parameters.width) * 0.5,
            dry: parameters.dry,
            room_size: 1.0,
            damp: 0.0,
        },
    }
}

mod tuning {
    // Freeverb model tuning from: https://github.com/sinshu/freeverb/blob/main/Components/tuning.h

    // Fixed gain to scale the input to the comb filters when not in freeze mode
    // This also helps keep the output of the reverb from exceeding unity gain
    pub const FIXED_GAIN: f32 = 0.03;

    // 0.0 = small room, to 1.0 = large room; maps to 0.7 to 0.98
    pub const SCALE_ROOM: f32 = 0.28;
    pub const OFFSET_ROOM: f32 = 0.7;
    pub const INITIAL_ROOM: f32 = 0.5;

    // 0.0 = no damping, 1.0 = full damping; maps to 0.0 to 0.4
    pub const INITIAL_DAMP: f32 = 0.5;
    pub const SCALE_DAMP: f32 = 0.4;

    pub const INITIAL_WET: f32 = 0.5; // 0.0 = no wet, 1.0 = full wet
    pub const INITIAL_DRY: f32 = 0.0; // 0.0 = no dry, 1.0 = full dry
    pub const INITIAL_WIDTH: f32 = 0.0; // 0.0 = mono, 1.0 = stereo

    pub const ALLPASS_FEEDBACK: f32 = 0.5; // feedback coefficient for allpass filters

    // Spread between left and right delay times for stereo effect
    pub const STEREO_SPREAD_SEC: f32 = 0.000_521_541_9; // 23 samples (at 44.1kHz)

    pub const NUM_COMBS: usize = 8;
    pub const COMB_SECOND_TUNINGS: [f32; NUM_COMBS] = [
        0.025_306_122, // 1116 samples at 44.1kHz
        0.026_938_775, // 1188 samples at 44.1kHz
        0.028_956_916, // 1277 samples at 44.1kHz
        0.030_748_299, // 1356 samples at 44.1kHz
        0.032_244_9,   // 1422 samples at 44.1kHz
        0.033_809_524, // 1491 samples at 44.1kHz
        0.035_306_122, // 1557 samples at 44.1kHz
        0.036_666_666, // 1617 samples at 44.1kHz
    ];

    pub const NUM_ALLPASS: usize = 4;
    pub const ALLPASS_SECOND_TUNINGS: [f32; NUM_ALLPASS] = [
        0.012_607_709_7, // 556 samples at 44.1kHz
        0.01,            // 441 samples at 44.1kHz
        0.007_732_426_3, // 341 samples at 44.1kHz
        0.005_102_040_8, // 225 samples at 44.1kHz
    ];
}
