use crate::PCM;

const INITIAL_SAMPLE_RATE: usize = 48_000;

pub struct DcBlock<S: PCM> {
    last_input: S,
    last_output: S,
    gain: f32,
}

impl<S: PCM> DcBlock<S> {
    pub const fn new(sample_rate: usize) -> Self {
        Self {
            last_input: S::PCM_EQUILIBRIUM,
            last_output: S::PCM_EQUILIBRIUM,
            gain: 1.0 - 10.0 / sample_rate as f32,
        }
    }

    pub fn prepare(&mut self, sample_rate: usize) {
        self.gain = 1.0 - 10.0 / sample_rate as f32;
    }
}

impl DcBlock<f32> {
    pub fn tick(&mut self, input: &f32) -> f32 {
        let out = input - self.last_input + (self.gain * self.last_output);
        self.last_input = *input;
        self.last_output = out;
        out
    }
}

impl<S: PCM> Default for DcBlock<S> {
    fn default() -> Self {
        DcBlock::new(INITIAL_SAMPLE_RATE)
    }
}
