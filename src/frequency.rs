use std::f64::consts::PI;
use crate::magnitude::*;

/// Frequency magnitude, in hertz.
pub struct Frequency(f64);

impl Magnitude for Frequency {
    fn si(val: f64) -> Self { Frequency(val) }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "Hz".to_owned() }
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