use crate::{PCM, Stereo, components::DelayLine};

pub struct SimpleDelay<S: PCM, const N: usize> {
    left: DelayLine<S, N>,
    right: DelayLine<S, N>,
    feedback: f32,
}

impl<S: PCM, const N: usize> SimpleDelay<S, N> {
    pub const fn new(feedback: f32) -> Self {
        Self {
            left: DelayLine::const_default(),
            right: DelayLine::const_default(),
            feedback,
        }
    }

    pub const fn const_default() -> Self {
        Self {
            left: DelayLine::const_default(),
            right: DelayLine::const_default(),
            feedback: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }

    #[inline(always)]
    pub fn tick(&mut self, input: &Stereo<f32>) -> Stereo<f32> {
        let left = input[0] + self.left.peek().into() * self.feedback;
        self.left.write(S::from(left));
        self.left.advance();

        let right = input[1] + self.right.peek().into() * self.feedback;
        self.right.write(S::from(right));
        self.right.advance();

        [left, right]
    }

    pub fn set_feedback(&mut self, val: f32) {
        self.feedback = val;
    }

    pub fn set_delay(&mut self, sec: f32, sample_rate: usize) {
        self.left.set_length(sec, sample_rate);
        self.right.set_length(sec, sample_rate);
    }
}

impl<S: PCM, const N: usize> Default for SimpleDelay<S, N> {
    fn default() -> Self {
        Self::const_default()
    }
}
