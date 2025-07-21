use crate::PCM;
use crate::components::DelayLine;

const ALLPASS_FEEDBACK: f32 = 0.5;

/// Schroeder all-pass filter approximation with a maximum of `N` samples in the delay line.
///
/// It is an approximation using an FBCF and FFCF in series. Only a true all-pass for `feedback = 0.5`.
/// A complete analysis can be found [here](https://www.dsprelated.com/freebooks/pasp/Freeverb.html)
#[derive(Debug, Copy, Clone)]
pub struct AllPass<S: PCM, const N: usize> {
    line: DelayLine<S, N>,
}

impl<S: PCM, const N: usize> AllPass<S, N> {
    /// Default const constructor, i.e. can be created at compile-time.
    pub const fn const_default() -> Self {
        Self {
            line: DelayLine::const_default(),
        }
    }

    #[inline(always)]
    pub fn tick(&mut self, input: &f32) -> f32 {
        let delay_line: f32 = self.line.peek().into();

        // update delay line
        let delay_input = input + delay_line * ALLPASS_FEEDBACK;
        self.line.write(S::from(delay_input));
        self.line.advance();

        delay_line - input
    }

    /// Reset the allpass filter by clearing the underlying delay line.
    pub fn reset(&mut self) {
        self.line.reset();
    }

    /// Set the delay in seconds.
    pub fn set_delay(&mut self, seconds: f32, sample_rate: usize) {
        self.line.set_length(seconds, sample_rate);
    }
}

impl<S: PCM, const N: usize> Default for AllPass<S, N> {
    fn default() -> Self {
        Self::const_default()
    }
}
