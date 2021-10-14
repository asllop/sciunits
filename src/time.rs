use std::ops::Div;
use crate::{magnitude::Magnitude, frequency::Frequency};

/// Time magnitude, in seconds.
pub struct Time(f64);

impl Magnitude for Time {
    fn si(val: f64) -> Self { if val < 0f64 { Time(f64::NAN) } else { Time(val) } }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "s".to_owned() }
}

// The inverse of Time is Frequency.
impl Div<Time> for i32 {
    type Output = Frequency;

    fn div(self, rhs: Time) -> Self::Output {
        Frequency::si(self as f64 / rhs.val())
    }
}

impl_magnitude_generics!(Time);