use crate::PCM;

use core::cmp::min;

/// Fixed-length delay line with a capacity of `N` samples.
///
/// # Examples
///
/// ```
/// use dspkit::components::DelayLine;
///
/// let mut line = DelayLine::new([-1.0f32, 1.0f32], 1);
///
/// assert_eq!(line.peek(), -1.0);
///
/// line.write(0.5f32);
/// line.advance();
///
/// assert_eq!(line.peek(), 0.5); // Since size=1, we should be back at the first element.
/// ```
#[derive(Debug, Copy, Clone)]
pub struct DelayLine<S: PCM, const N: usize> {
    buffer: [S; N],
    index: usize,
    size: usize,
}

impl<S: PCM, const N: usize> DelayLine<S, N> {
    /// Construct a new delay line from an audio buffer array. The `size` parameter determines the effective size
    /// of the delay line.
    pub const fn new(buffer: [S; N], size: usize) -> Self {
        DelayLine {
            buffer,
            index: 0,
            size,
        }
    }

    /// Default const constructor, i.e. can be constructed at compile-time.
    /// ```
    /// use dspkit::components::DelayLine;
    ///
    /// static LINE: DelayLine<f32, 1024> = DelayLine::const_default();
    /// ```
    pub const fn const_default() -> Self {
        Self::new([S::PCM_EQUILIBRIUM; N], N)
    }

    /// Reset the delay line, moving to the starting index and clearing the audio buffer.
    ///
    /// ```
    /// use dspkit::PCM;
    /// use dspkit::components::DelayLine;
    ///
    /// let mut line = DelayLine::new([0.5f32; 1024], 1024);
    /// line.reset();
    ///
    /// assert!(line.into_iter().take(1024).all(|x| x == f32::PCM_EQUILIBRIUM))
    /// ```
    pub fn reset(&mut self) {
        self.index = 0;
        self.buffer.fill(S::PCM_EQUILIBRIUM);
    }

    /// Zero the delay line by applying the [`PCM::PCM_EQUILIBRIUM`] signal to the specified range.
    #[inline(always)]
    pub fn zero(&mut self, start: usize, end: usize) {
        let end = min(end, min(self.buffer.len(), self.size));
        self.buffer[start..end].fill(S::PCM_EQUILIBRIUM);
    }

    /// Set the delay line length in seconds, given some sample rate. Overflows will be clamped to
    /// the capacity of the audio buffer.
    #[inline(always)]
    pub fn set_length(&mut self, sec: f32, sample_rate: usize) {
        let new_length = (sec * sample_rate as f32) as usize;
        let new_length = min(new_length, self.buffer.len());

        // Zero out any new values added to the buffer
        // if self.size < new_length {
        //      self.buffer[self.size..new_length].fill(S::PCM_EQUILIBRIUM);
        // }

        self.size = new_length;
        if self.index >= self.size {
            self.index = 0;
        }
    }

    /// The maximum capacity of audio samples.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Write the value to the current index.
    #[inline(always)]
    pub fn write(&mut self, val: S) {
        self.buffer[self.index] = val;
    }

    /// Read the value at the current index.
    #[inline(always)]
    pub fn peek(&self) -> S {
        self.buffer[self.index]
    }

    /// Advance the delay line, wrapping as a circular buffer if necessary.
    #[inline(always)]
    pub fn advance(&mut self) {
        self.index += 1;
        if self.index >= self.size {
            self.index = 0;
        }
    }
}

impl<S: PCM, const N: usize> Iterator for DelayLine<S, N> {
    type Item = S;
    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        Some(self.peek())
    }
}

impl<S: PCM, const N: usize> Default for DelayLine<S, N> {
    /// Returns a default delay-line with a silent signal.
    ///
    /// ```
    /// use dspkit::PCM;
    /// use dspkit::components::DelayLine;
    ///
    /// let line: DelayLine<f32, 1024> = DelayLine::default();
    /// assert!(line.into_iter().take(1024).all(|x| x == f32::PCM_EQUILIBRIUM))
    /// ```
    fn default() -> Self {
        Self::const_default()
    }
}
