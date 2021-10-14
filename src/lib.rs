use std::ops::Div;

#[macro_export]
macro_rules! impl_mag_display {
    ( $x:ty ) => {
        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.val(), self.units())
            }
        }
    };
}

pub trait Magnitude {
    /// Build a new magnitude
    fn new(val: f64) -> Self where Self: Sized;
    /// Magnitude in SI units.
    fn val(&self) -> f64;
    /// SI units of current magnitude.
    fn units(&self) -> String;
}

pub struct Time(f64);

impl Magnitude for Time {
    fn new(val: f64) -> Self { Time(val) }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "s".to_owned() }
}

impl_mag_display!(Time);

pub struct Frequency(f64);

impl Magnitude for Frequency {
    fn new(val: f64) -> Self { Frequency(val) }
    fn val(&self) -> f64 { self.0 }
    fn units(&self) -> String { "Hz".to_owned() }
}

impl_mag_display!(Frequency);

impl Div<Time> for f64 {
    type Output = Frequency;

    fn div(self, rhs: Time) -> Self::Output {
        Frequency(self / rhs.val())
    }
}

impl Div<Frequency> for f64 {
    type Output = Time;

    fn div(self, rhs: Frequency) -> Self::Output {
        Time(self / rhs.val())
    }
}

