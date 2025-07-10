use crate::core::{AudioBuffer, ConstDefault, PCM, Process};

pub struct CombFilter<S: PCM, const N: usize> {
    mix: f32,
    feedback: f32,
    audio_buffer: AudioBuffer<S, N>,
}

impl<const N: usize> Process<f32, f32> for CombFilter<f32, N> {
    #[inline(always)]
    fn process(&mut self, input: &f32) -> f32 {
        // compute new wet signal
        let delay_line = *self.audio_buffer.peek();
        let wet = input + delay_line * self.feedback;

        // update delay line
        self.audio_buffer.write_and_advance(wet);

        wet * self.mix + (1.0 - self.mix) * input
    }
}

impl<S: PCM, const N: usize> CombFilter<S, N> {
    pub const fn new(mix: f32, feedback: f32, audio_buffer: AudioBuffer<S, N>) -> Self {
        CombFilter {
            mix,
            feedback,
            audio_buffer,
        }
    }

    pub const fn const_default() -> Self {
        Self::new(0.0, 0.0, AudioBuffer::DEFAULT)
    }

    #[inline(always)]
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix;
    }

    #[inline(always)]
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    #[inline(always)]
    pub fn set_delay(&mut self, seconds: f32, sample_rate: usize) {
        self.audio_buffer.set_length(seconds, sample_rate);
    }

    pub fn init(&mut self) {
        self.audio_buffer.init();
    }
}

impl<S: PCM, const N: usize> ConstDefault for CombFilter<S, N> {
    const DEFAULT: Self = Self::const_default();
}
