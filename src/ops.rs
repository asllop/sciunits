use std::ops::Div;
use crate::{time::*, frequency::*, magnitude::*};

/// The inverse of Time is Frequency.
impl Div<Time> for f64 {
    type Output = Frequency;

    fn div(self, rhs: Time) -> Self::Output {
        Frequency::si(self / rhs.val())
    }
}

/// The inverse of Frequency is Time.
impl Div<Frequency> for f64 {
    type Output = Time;

    fn div(self, rhs: Frequency) -> Self::Output {
        Time::si(self / rhs.val())
    }
}
