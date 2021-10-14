use std::ops::Div;
use crate::{time::*, frequency::*, magnitude::*};

/// The inverse of Time is Frequency.
impl Div<Time> for i32 {
    type Output = Frequency;

    fn div(self, rhs: Time) -> Self::Output {
        Frequency::si(self as f64 / rhs.val())
    }
}

/// The inverse of Frequency is Time.
impl Div<Frequency> for i32 {
    type Output = Time;

    fn div(self, rhs: Frequency) -> Self::Output {
        Time::si(self as f64 / rhs.val())
    }
}
