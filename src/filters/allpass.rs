use crate::core::{AudioBuffer, ConstDefault, PCM, Process};

pub struct AllPass<S: PCM, const N: usize> {
    feedback: f32,
    audio_buffer: AudioBuffer<S, N>,
}

impl<const N: usize> Process<f32, f32> for AllPass<f32, N> {
    #[inline(always)]
    fn process(&mut self, input: &f32) -> f32 {
        let feedback = self.feedback;
        let delay_line = *self.audio_buffer.peek();

        // update delay line
        let delay_input = input + delay_line * feedback;
        self.audio_buffer.write_and_advance(delay_input);

        delay_line - delay_input * feedback
    }
}

impl<S: PCM, const N: usize> AllPass<S, N> {
    pub const fn new(feedback: f32, audio_buffer: AudioBuffer<S, N>) -> Self {
        Self {
            feedback,
            audio_buffer,
        }
    }

    pub const fn const_default() -> Self {
        Self::new(1.0, AudioBuffer::DEFAULT)
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

impl<S: PCM, const N: usize> ConstDefault for AllPass<S, N> {
    const DEFAULT: Self = Self::const_default();
}
