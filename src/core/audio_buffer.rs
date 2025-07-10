use super::ConstDefault;
use super::pcm::PCM;

use core::cmp::min;

pub struct AudioBuffer<S: PCM, const N: usize> {
    buffer: [S; N],
    index: usize,
    size: usize,
}

impl<S: PCM, const N: usize> AudioBuffer<S, N> {
    pub const fn new(buffer: [S; N], size: usize) -> Self {
        AudioBuffer {
            buffer,
            index: 0,
            size,
        }
    }

    pub const fn const_default() -> Self {
        Self::new([S::PCM_EQUILIBRIUM; N], N)
    }

    pub fn init(&mut self) {
        self.index = 0;
        self.zero(0, self.buffer.len())
    }

    #[inline(always)]
    pub fn zero(&mut self, start: usize, end: usize) {
        let end = end.min(self.size).min(self.buffer.len());
        self.buffer[start..end].fill(PCM::PCM_EQUILIBRIUM);
    }

    #[inline(always)]
    pub fn set_length(&mut self, sec: f32, sample_rate: usize) {
        let new_length = (sec * sample_rate as f32) as usize;
        let new_length = min(new_length, self.buffer.len());

        // Zero out any new values added to the buffer
        // if self.size < new_length {
        //     self.zero(self.size, new_length);
        // }

        self.size = new_length;
        if self.size > self.index {
            self.index = self.size;
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    #[inline(always)]
    pub fn peek(&self) -> &S {
        &self.buffer[self.index]
    }

    #[inline(always)]
    pub fn write_and_advance(&mut self, val: S) {
        self.buffer[self.index] = val;
        self.index += 1;
        if self.index >= self.size {
            self.index = 0;
        }
    }
}

impl<S: PCM, const N: usize> ConstDefault for AudioBuffer<S, N> {
    const DEFAULT: Self = Self::const_default();
}

#[inline(always)]
pub const fn compute_buffer_length(seconds: f32, sample_rate: usize) -> usize {
    (seconds * sample_rate as f32) as usize
}
