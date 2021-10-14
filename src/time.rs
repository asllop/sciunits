use crate::magnitude::*;

/// Time magnitude, in seconds.
pub struct Time(f64);

impl Magnitude for Time {
    fn si(val: f64) -> Self { if val < 0f64 { Time(f64::NAN) } else { Time(val) } }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "s".to_owned() }
}

impl_magnitude_generics!(Time);