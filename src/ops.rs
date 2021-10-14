use std::ops::Div;
use crate::{time::*, frequency::*, magnitude::*};

impl Div<Time> for f64 {
    type Output = Frequency;

    fn div(self, rhs: Time) -> Self::Output {
        Frequency::si(self / rhs.val())
    }
}

impl Div<Frequency> for f64 {
    type Output = Time;

    fn div(self, rhs: Frequency) -> Self::Output {
        Time::si(self / rhs.val())
    }
}
