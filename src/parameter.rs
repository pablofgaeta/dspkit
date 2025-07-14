// use libm::expf;
//
// fn smooth_logistic(x: f32) {}
//
// // Core trait for curve transformations
// pub trait CurveTransform {
//     fn apply(&self, x: f32) -> f32;
//     fn apply_unchecked(&self, x: f32) -> f32 {
//         self.apply(x)
//     }
// }
//
// // Enum for common curve types
// #[derive(Debug, Clone)]
// pub enum Curve {
//     Linear,
//     Exponential { base: f32 },
//     Logarithmic { base: f32 },
//     Power { exponent: f32 },
//     SCurve { steepness: f32 },
// }
//
// impl CurveTransform for Curve {
//     fn apply(&self, x: f32) -> f32 {
//         match self {
//             Curve::Linear => x,
//             Curve::Exponential { base } => (base.powf(x) - 1.0) / (base - 1.0),
//             Curve::Power { exponent } => x.powf(*exponent),
//             Curve::Logarithmic { base } => (x * (base - 1.0) + 1.0).ln() / base.ln(),
//             Curve::SCurve { steepness } => {
//                 // Sigmoid-like curve using tanh
//                 let scaled = (x - 0.5) * steepness;
//                 (scaled.tanh() + 1.0) * 0.5
//             } // ... other implementations
//         }
//     }
// }
//
// // For custom curves
// pub struct CustomCurve<F>
// where
//     F: Fn(f32) -> f32,
// {
//     func: F,
// }
//
// impl<F> Curve for CustomCurve<F>
// where
//     F: Fn(f32) -> f32,
// {
//     fn apply(&self, x: f32) -> f32 {
//         (self.func)(x)
//     }
// }
//
// // Convenience constructors
// impl CurveType {
//     pub fn exponential(strength: f32) -> Self {
//         Self::Exponential {
//             base: 2.0_f32.powf(strength),
//         }
//     }
//
//     pub fn logarithmic(strength: f32) -> Self {
//         Self::Logarithmic {
//             base: 2.0_f32.powf(strength),
//         }
//     }
// }
//
// fn check() {
//     CurveType::exponential(1.0).apply()
// }
//
// /// Computes x on a logistic curve. Maps [0, 1] -> (0, 1) on an S-curve.
// ///
// /// # Examples
// ///
// /// ```
// /// use dspkit::logistic_0to1;
// /// assert!(logistic_0to1(0.25) < 0.25);
// /// assert!(logistic_0to1(0.75) > 0.25);
// /// ```
// #[inline(always)]
// pub fn logistic_0to1(x: f32) -> f32 {
//     1.0 / expf(1.0 + (-10.0 * (x - 0.5)))
// }
//
// pub struct Parameter<C: Curve> {
//     curve: C,
//     smoother: Option<Box<dyn Smoother>>,
//     raw_value: f32,
//     smoothed_value: f32,
// }
//
// impl<C: Curve> Parameter<C> {
//     pub fn with_smoothing(curve: C, smoother: impl Smoother + 'static) -> Self {
//         Self {
//             curve,
//             smoother: Some(Box::new(smoother)),
//             raw_value: 0.0,
//             smoothed_value: 0.0,
//         }
//     }
//
//     pub fn update(&mut self, new_value: f32, delta_time: f32) -> f32 {
//         self.raw_value = new_value;
//         let curved = self.curve.apply(new_value);
//
//         if let Some(smoother) = &mut self.smoother {
//             self.smoothed_value = smoother.smooth(curved, delta_time);
//         } else {
//             self.smoothed_value = curved;
//         }
//
//         self.smoothed_value
//     }
// }
