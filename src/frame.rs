use crate::PCM;

/// Stereo frame. Fixed, 2 sample array representing left and right channels.
pub type Stereo<S> = [S; 2];

/// Mono frame. Fixed, 1 sample array representing single channel.
pub type Mono<S> = [S; 1];

/// A single frame of sample channels.
pub trait Frame<S: PCM> {
    /// The number of channels that this frame contains.
    const NUM_CHANNELS: usize;

    /// Access a slice of samples from the frame.
    fn as_slice(&self) -> &[S];

    /// Access a **mutable** slice of samples from the frame.
    fn as_slice_mut(&mut self) -> &mut [S];
}

impl<const N: usize> Frame<f32> for [f32; N] {
    const NUM_CHANNELS: usize = N;

    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

/// Allows conversion to a single-channel sample.
pub trait ToMono<S: PCM> {
    /// Generate a single-channel sample.
    fn to_mono(&self) -> S;
}

impl<const N: usize> ToMono<f32> for [f32; N] {
    #[inline(always)]
    fn to_mono(&self) -> f32 {
        self.iter().sum::<f32>() / (self.len() as f32)
    }
}
