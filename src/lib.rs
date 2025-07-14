#![no_std]

pub mod components;
pub mod effects;
mod frame;
mod parameter;
mod pcm;

pub use frame::{Frame, Mono, Stereo, ToMono};
pub use pcm::PCM;

/// An audio node which can process individual or batches of samples.
pub trait AudioNode<I, O> {
    /// Prepare the audio node before processing.
    #[allow(unused_variables)]
    fn prepare(&mut self, sample_rate: usize) {}

    /// Process a single sample.
    fn tick(&mut self, input: &I) -> O;

    /// Process a batch of samples.
    fn batch(&mut self, input: &[I], output: &mut [O]) {
        for (idx, val) in input.iter().enumerate() {
            output[idx] = self.tick(val);
        }
    }
}
