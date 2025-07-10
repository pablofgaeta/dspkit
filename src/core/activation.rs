use libm::expf;

/// Computes x on a logistic curve, assuming x is bound by (0, 1)
#[inline(always)]
pub fn logistic_0to1(x: f32) -> f32 {
    1.0 / expf(1.0 + (-10.0 * (x - 0.5)))
}

/// Computes x on a logistic curve, assuming x is bound by a minimum and maximum steepness.
#[inline(always)]
pub fn bounded_logistic(x: f32, min_val: f32, max_val: f32, steepness: f32) -> f32 {
    let diff = max_val - min_val;
    let center = diff / 2.0;

    min_val + (diff / (1.0 + expf(-steepness * (x - center))))
}
