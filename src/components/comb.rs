use crate::PCM;
use crate::components::DelayLine;

/// Comb filter with a maximum of `N` samples in the delay line.
#[derive(Debug, Copy, Clone)]
pub struct CombFilter<S: PCM, const N: usize> {
    mix: f32,
    feedback: f32,
    line: DelayLine<S, N>,
}

impl<S: PCM, const N: usize> CombFilter<S, N> {
    /// Construct a comb filter with the specified feedback coefficient and mix for the wet signal.
    ///
    /// Asserts: `0 <= mix <= 1` and `0 <= feedback <= 1`.
    pub fn new(mix: f32, feedback: f32) -> Self {
        assert!((0.0..=1.0).contains(&mix));
        assert!((0.0..=1.0).contains(&feedback));

        CombFilter {
            mix,
            feedback,
            line: DelayLine::const_default(),
        }
    }

    #[inline(always)]
    pub fn tick(&mut self, input: &f32) -> f32 {
        // compute new wet signal
        let delay_line: f32 = self.line.peek().into();
        let wet = input + delay_line * self.feedback;

        // update delay line
        self.line.write(S::from(wet));
        self.line.advance();

        wet * self.mix + (1.0 - self.mix) * input
    }

    /// Default const constructor, i.e. can be created at compile-time.   
    /// ```
    /// use dspkit::components::CombFilter;
    ///
    /// static LINE: CombFilter<f32, 1024> = CombFilter::const_default();
    /// ```
    pub const fn const_default() -> Self {
        CombFilter {
            mix: 0.0,
            feedback: 0.0,
            line: DelayLine::const_default(),
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        assert!((0.0..=1.0).contains(&mix));

        self.mix = mix;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        assert!((0.0..=1.0).contains(&feedback));

        self.feedback = feedback;
    }

    #[inline(always)]
    pub fn set_delay(&mut self, seconds: f32, sample_rate: usize) {
        self.line.set_length(seconds, sample_rate);
    }

    /// Reset the comb filter by clearing the underlying delay line.
    pub fn reset(&mut self) {
        self.line.reset();
    }
}
