use crate::traits::{Rollable, RollableUnivariate, Univariate};
use num::{Float, FromPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, SubAssign};
/// Running sum.
/// # Examples
/// ```
/// use online_statistics::traits::Univariate;
/// use online_statistics::sum::Sum;
/// let mut running_sum: Sum<f64> = Sum::new();
/// for i in 1..10{
///     running_sum.update(i as f64);
/// }
/// assert_eq!(running_sum.get(), 45.0);
/// ```
///
#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Sum<F: Float + FromPrimitive + AddAssign + SubAssign> {
    pub sum: F,
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Sum<F> {
    pub fn new() -> Self {
        Self {
            sum: F::from_f64(0.0).unwrap(),
        }
    }
}
impl<F: Float + FromPrimitive + AddAssign + SubAssign> Univariate<F> for Sum<F> {
    fn update(&mut self, x: F) {
        self.sum += x;
    }
    fn get(&self) -> F {
        self.sum
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> Rollable<F> for Sum<F> {
    fn revert(&mut self, x: F) -> std::result::Result<(), &'static str> {
        self.sum -= x;
        Ok(())
    }
}

impl<F: Float + FromPrimitive + AddAssign + SubAssign> RollableUnivariate<F> for Sum<F> {}
