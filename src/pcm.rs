/// PCM audio encoding representation.
pub trait PCM: Copy + Clone + PartialOrd + From<f32> + Into<f32> {
    /// Represents the lowest possible PCM value.
    const PCM_LOW: Self;

    /// Represents the highest possible PCM value.
    const PCM_HIGH: Self;

    /// Represents a "silent" signal for the audio encoding.
    const PCM_EQUILIBRIUM: Self;

    /// Clamp PCM signal within the valid range.
    fn constrain(self) -> Self {
        if self < Self::PCM_LOW {
            Self::PCM_LOW
        } else if self > Self::PCM_HIGH {
            Self::PCM_HIGH
        } else {
            self
        }
    }
}

impl PCM for f32 {
    const PCM_LOW: Self = -1.0;
    const PCM_HIGH: Self = -1.0;
    const PCM_EQUILIBRIUM: Self = 0.0;
}
