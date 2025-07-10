use crate::core::{ConstDefault, Process};

pub struct Vca {
    amplitude: f32,
}

impl Process<f32, f32> for Vca {
    #[inline(always)]
    fn process(&mut self, input: &f32) -> f32 {
        input * self.amplitude
    }
}

impl Vca {
    pub const fn new(amplitude: f32) -> Self {
        Self { amplitude }
    }

    pub const fn const_default() -> Self {
        Self::new(1.0)
    }

    #[inline(always)]
    pub fn set_amplitude(&mut self, val: f32) {
        self.amplitude = val;
    }
}

impl ConstDefault for Vca {
    const DEFAULT: Self = Self::const_default();
}
