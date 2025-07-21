use crate::PCM;
use crate::components::DelayLine;

/// Lowpass feedback comb filter with a maximum of `N` samples in the delay line.
///
/// The delay line is lowpass-filtered and summed with the input signal.
/// The low-pass filtering is a unity-gain one-pole low-pass.
/// A complete analysis can be found [here](https://www.dsprelated.com/freebooks/pasp/Freeverb.html)
#[derive(Debug, Copy, Clone)]
pub struct Comb<S: PCM, const N: usize> {
    mix: f32,
    feedback: f32,
    lp_signal: S,
    line: DelayLine<S, N>,
}

impl<S: PCM, const N: usize> Comb<S, N> {
    #[inline(always)]
    pub fn tick(&mut self, input: &f32) -> f32 {
        let output: f32 = self.line.peek().into();

        // Update using unity-gain one-pole lowpass filter on output signal.
        let lp_signal = self.mix * self.lp_signal.into() + (1.0 - self.mix) * output;
        self.lp_signal = S::from(lp_signal);

        // Update delay line
        self.line.write(S::from(input + self.feedback * lp_signal));
        self.line.advance();

        output
    }

    /// Default const constructor, i.e. can be created at compile-time.   
    pub const fn const_default() -> Self {
        Comb {
            mix: 0.0,
            feedback: 0.0,
            lp_signal: S::PCM_EQUILIBRIUM,
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

    pub fn set_delay(&mut self, seconds: f32, sample_rate: usize) {
        self.line.set_length(seconds, sample_rate);
    }

    /// Reset the comb filter by clearing the underlying delay line.
    pub fn reset(&mut self) {
        self.line.reset();
    }
}
