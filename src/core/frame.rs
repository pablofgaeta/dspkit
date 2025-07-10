use super::pcm::PCM;

pub type Stereo<S> = [S; 2];
pub type Mono<S> = [S; 1];

pub trait Frame<S: PCM> {
    const NUM_CHANNELS: usize;

    fn samples(&self) -> &[S];
    fn samples_mut(&mut self) -> &mut [S];
}

pub trait ToMono<S: PCM> {
    fn to_mono(&self) -> S;
}

impl Frame<f32> for Mono<f32> {
    const NUM_CHANNELS: usize = 1;

    fn samples(&self) -> &[f32] {
        self.as_slice()
    }

    fn samples_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

impl<S: PCM> Frame<S> for Stereo<S> {
    const NUM_CHANNELS: usize = 2;

    fn samples(&self) -> &[S] {
        self.as_slice()
    }

    fn samples_mut(&mut self) -> &mut [S] {
        self.as_mut_slice()
    }
}

impl ToMono<f32> for Mono<f32> {
    #[inline(always)]
    fn to_mono(&self) -> f32 {
        self[0]
    }
}

impl ToMono<f32> for Stereo<f32> {
    #[inline(always)]
    fn to_mono(&self) -> f32 {
        0.5 * (self[0] + self[1])
    }
}
