use core::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct U24(u32);

impl U24 {
    pub fn new(val: u32) -> Self {
        Self(val)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

pub trait PCM: Clone {
    const PCM_EQUILIBRIUM: Self;
}

impl PCM for f32 {
    const PCM_EQUILIBRIUM: Self = 0.0;
}

impl PCM for U24 {
    const PCM_EQUILIBRIUM: Self = U24(0);
}

impl From<f32> for U24 {
    fn from(value: f32) -> Self {
        // Clamp to valid range
        let clamped = value.clamp(-1.0, 1.0);

        // Convert to 24-bit signed integer, scale by 2^23 - 1
        let int_sample = (clamped * 8_388_607.0) as i32;

        // Pack into u32
        U24(int_sample as u32)
    }
}

impl From<U24> for f32 {
    fn from(value: U24) -> Self {
        // Sign extend by shifting left then right as signed
        let signed_sample = (value.inner() << 8) as i32 >> 8;

        // Convert to float in range [-1, 1], scale by 2^23
        (signed_sample as f32 / 8_388_608.0).clamp(-1.0, 1.0)
    }
}

impl PartialEq for U24 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for U24 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f32_to_u24_equilibrium() {
        assert_eq!(U24::from(f32::PCM_EQUILIBRIUM), U24::PCM_EQUILIBRIUM);
    }
}
