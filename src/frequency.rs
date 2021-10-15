use std::{ops::Div, f64::consts::PI};
use crate::{magnitude::Magnitude, time::Time, array::Array};

/// Frequency magnitude, in hertz.
#[derive(Clone)]
pub struct Frequency(f64);

impl Magnitude for Frequency {
    fn si(val: f64) -> Self { if val < 0f64 { Frequency(f64::NAN) } else { Frequency(val) } }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "Hz".to_owned() }
}

/// The inverse of Frequency is Time.
impl Div<Frequency> for i32 {
    type Output = Time;

    fn div(self, rhs: Frequency) -> Self::Output {
        Time::si(self as f64 / rhs.val())
    }
}

/// The inverse of Frequency array is a Time array.
impl Div<Array<Frequency>> for i32 {
    type Output = Array<Time>;

    fn div(self, rhs: Array<Frequency>) -> Self::Output {
        let mut v = Vec::new();
        for m in rhs.iter() {
            let m = self as f64 / m.val();
            v.push(Time::si(m));
        }
        crate::array::Array::new(v)
    }
}

impl_magnitude_generics!(Frequency);

impl Frequency {
    /// From radiants per second.
    pub fn from_rads(val: f64) -> Self {
        Frequency::si(val / (2.0*PI))
    }

    /// From revolutions per minute.
    pub fn from_rpm(val: f64) -> Self {
        Frequency::si(val / 60.0)
    }
}