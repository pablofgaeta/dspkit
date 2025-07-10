use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub trait AtomicVar<T> {
    fn set(&self, val: T);
    fn get(&self) -> T;
}

impl AtomicVar<usize> for AtomicUsize {
    #[inline]
    fn set(&self, val: usize) {
        self.store(val, Ordering::Relaxed)
    }

    #[inline]
    fn get(&self) -> usize {
        self.load(Ordering::Relaxed)
    }
}

pub struct AtomicF32 {
    storage: AtomicU32,
}

impl AtomicF32 {
    pub const fn new(val: f32) -> Self {
        let storage = AtomicU32::new(val.to_bits());
        Self { storage }
    }
}

impl AtomicVar<f32> for AtomicF32 {
    #[inline]
    fn set(&self, val: f32) {
        self.storage.store(val.to_bits(), Ordering::Relaxed)
    }

    #[inline]
    fn get(&self) -> f32 {
        f32::from_bits(self.storage.load(Ordering::Relaxed))
    }
}
