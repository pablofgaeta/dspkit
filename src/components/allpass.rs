use crate::components::DelayLine;
use crate::{AudioNode, PCM};

/// All-pass filter with a maximum of `N` samples in the delay line.
#[derive(Debug, Copy, Clone)]
pub struct AllPass<S: PCM, const N: usize> {
    feedback: f32,
    line: DelayLine<S, N>,
}

impl<Storage: PCM, const N: usize> AudioNode<f32, f32> for AllPass<Storage, N> {
    #[inline(always)]
    fn tick(&mut self, input: &f32) -> f32 {
        let feedback = self.feedback;
        let delay_line: f32 = self.line.peek().into();

        // update delay line
        let delay_input = input + delay_line * feedback;
        self.line.write(Storage::from(delay_input));
        self.line.advance();

        delay_line - delay_input * feedback
    }
}

impl<S: PCM, const N: usize> AllPass<S, N> {
    /// Construct a new all-pass filter with the given feedback coefficient.
    ///
    /// Asserts: `0 <= feedback <= 1`
    pub fn new(feedback: f32) -> Self {
        assert!((0.0..=1.0).contains(&feedback));

        Self {
            feedback,
            line: DelayLine::const_default(),
        }
    }

    /// Default const constructor, i.e. can be created at compile-time.   
    /// ```
    /// use dspkit::components::AllPass;
    ///
    /// static LINE: AllPass<f32, 1024> = AllPass::const_default();
    /// ```
    pub const fn const_default() -> Self {
        Self {
            feedback: 1.0,
            line: DelayLine::const_default(),
        }
    }

    /// Reset the allpass filter by clearing the underlying delay line.
    pub fn reset(&mut self) {
        self.line.reset();
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        assert!((0.0..=1.0).contains(&feedback));
        self.feedback = feedback;
    }

    /// Set the delay in seconds.
    #[inline(always)]
    pub fn set_delay(&mut self, seconds: f32, sample_rate: usize) {
        self.line.set_length(seconds, sample_rate);
    }
}

impl<S: PCM, const N: usize> Default for AllPass<S, N> {
    fn default() -> Self {
        Self::const_default()
    }
}
