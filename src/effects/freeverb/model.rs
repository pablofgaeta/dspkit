use super::tuning;
use crate::core::{ConstDefault, PCM, Prepare, Process, Stereo};
use crate::filters::{AllPass, CombFilter};

#[derive(Clone, Copy)]
#[repr(C)]
pub enum FreeverbMode {
    Active,
    Frozen,
}

#[derive(Clone, Copy)]
pub struct FreeverbParameters {
    mode: FreeverbMode,
    room_size: f32,
    damp: f32,
    wet: f32,
    dry: f32,
    width: f32,
}

struct FreeverbDerivedVars {
    gain: f32,
    wet_l: f32,
    wet_r: f32,
    dry: f32,
    room_size: f32,
    damp: f32,
}

pub struct Freeverb<S: PCM, const N: usize> {
    parameters: FreeverbParameters,
    derived: FreeverbDerivedVars,
    combs_l: [CombFilter<S, N>; tuning::NUM_COMBS],
    combs_r: [CombFilter<S, N>; tuning::NUM_COMBS],
    allpass_l: [AllPass<S, N>; tuning::NUM_ALLPASS],
    allpass_r: [AllPass<S, N>; tuning::NUM_ALLPASS],
}

impl<const N: usize> Process<Stereo<f32>, Stereo<f32>> for Freeverb<f32, N> {
    fn process(&mut self, input: &Stereo<f32>) -> Stereo<f32> {
        let mut out_l = f32::PCM_EQUILIBRIUM;
        let mut out_r = f32::PCM_EQUILIBRIUM;
        let mono_input = self.derived.gain * (input[0] + input[1]);

        for (comb_l, comb_r) in self.combs_l.iter_mut().zip(self.combs_r.iter_mut()) {
            out_l += comb_l.process(&mono_input);
            out_r += comb_r.process(&mono_input);
        }

        for (all_pass_l, all_pass_r) in self.allpass_l.iter_mut().zip(self.allpass_r.iter_mut()) {
            out_l = all_pass_l.process(&mono_input);
            out_r = all_pass_r.process(&mono_input);
        }

        let wet_l = out_l * self.derived.wet_l + out_r * self.derived.wet_r;
        let wet_r = out_l * self.derived.wet_r + out_r * self.derived.wet_l;

        out_l = wet_l + input[0] * self.derived.dry;
        out_r = wet_r + input[1] * self.derived.dry;

        [out_l, out_r]
    }

    fn batch(&mut self, _input: &[Stereo<f32>], _output: &mut [Stereo<f32>]) {
        todo!()
    }
}

impl<S: PCM, const N: usize> Prepare for Freeverb<S, N> {
    #[inline]
    fn prepare(&mut self, sample_rate: usize) {
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
}

impl<const N: usize> Freeverb<f32, N> {
    pub fn process_stereo(&mut self, input: &Stereo<f32>) -> Stereo<f32> {
        self.process(input)
    }

    pub fn prepare_effect(&mut self, sample_rate: usize) {
        self.prepare(sample_rate);
    }
}

impl<S: PCM, const N: usize> Freeverb<S, N> {
    /// TODO: Implement type states to require calling prepare/reset before processing.
    pub const fn new(parameters: FreeverbParameters) -> Self {
        Self {
            parameters,
            derived: compute_derived_parameters(parameters),
            combs_l: [CombFilter::DEFAULT; tuning::NUM_COMBS],
            combs_r: [CombFilter::DEFAULT; tuning::NUM_COMBS],
            allpass_l: [AllPass::DEFAULT; tuning::NUM_ALLPASS],
            allpass_r: [AllPass::DEFAULT; tuning::NUM_ALLPASS],
        }
    }

    pub const fn const_default() -> Self {
        Self::new(FreeverbParameters::DEFAULT)
    }

    #[inline(always)]
    pub fn set_room_size(&mut self, val: f32) {
        self.parameters.room_size = val;
    }

    #[inline(always)]
    pub fn set_damp(&mut self, val: f32) {
        self.parameters.damp = val;
    }

    #[inline(always)]
    pub fn set_wet(&mut self, val: f32) {
        self.parameters.wet = val;
    }

    #[inline(always)]
    pub fn set_dry(&mut self, val: f32) {
        self.parameters.dry = val;
    }

    #[inline(always)]
    pub fn set_width(&mut self, val: f32) {
        self.parameters.width = val;
    }

    #[inline(always)]
    pub fn set_mode(&mut self, val: FreeverbMode) {
        self.parameters.mode = val;
    }

    pub fn set_parameters(&mut self, params: FreeverbParameters) {
        self.parameters = params;
    }

    pub fn init(&mut self) -> &mut Self {
        for comb in self.combs_l.iter_mut() {
            comb.init();
        }

        for comb in self.combs_r.iter_mut() {
            comb.init();
        }

        for allpass in self.allpass_l.iter_mut() {
            allpass.init();
        }

        for allpass in self.allpass_r.iter_mut() {
            allpass.init();
        }

        self
    }
}

impl<S: PCM, const N: usize> ConstDefault for Freeverb<S, N> {
    const DEFAULT: Self = Self::const_default();
}

impl ConstDefault for FreeverbParameters {
    const DEFAULT: Self = FreeverbParameters {
        mode: FreeverbMode::Active,
        room_size: tuning::INITIAL_ROOM,
        damp: tuning::INITIAL_DAMP,
        wet: tuning::INITIAL_WET,
        dry: tuning::INITIAL_DRY,
        width: tuning::INITIAL_WIDTH,
    };
}

impl Default for FreeverbParameters {
    fn default() -> Self {
        Self::DEFAULT
    }
}

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
