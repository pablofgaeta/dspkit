const INITIAL_SAMPLE_RATE: usize = 48_000;

pub struct Clock {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    phase_delta: f32,
}

impl Clock {
    pub const fn new(frequency: f32, sample_rate: usize) -> Self {
        let sample_rate = sample_rate as f32;
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
            phase_delta: frequency / sample_rate,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn prepare(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate as f32;
        self.phase_delta = self.frequency / self.sample_rate;
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase_delta = self.frequency / self.sample_rate;
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new(INITIAL_SAMPLE_RATE as f32, INITIAL_SAMPLE_RATE)
    }
}
