use std::ops::Div;
use crate::{magnitude::Magnitude, frequency::Frequency, array::Array};

/// Time magnitude, in seconds.
#[derive(Clone)]
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

/// The inverse of Time array is a Frequency array.
impl Div<Array<Time>> for i32 {
    type Output = Array<Frequency>;

    fn div(self, rhs: Array<Time>) -> Self::Output {
        let mut v = Vec::new();
        for m in rhs.iter() {
            let m = self as f64 / m.val();
            v.push(Frequency::si(m));
        }
        crate::array::Array::new(v)
    }
}

impl_magnitude_generics!(Time);

impl Time {
    /// From nanoseconds.
    pub fn from_ns(val: f64) -> Self {
        Time::si(val / 1_000_000_000.0)
    }

    /// To nanoseconds.
    pub fn to_ns(&self) -> f64 {
        self.val() * 1_000_000_000.0
    }

    /// From microseconds.
    pub fn from_us(val: f64) -> Self {
        Time::si(val / 1_000_000.0)
    }

    /// To microseconds.
    pub fn to_us(&self) -> f64 {
        self.val() * 1_000_000.0
    }

    /// From milliseconds.
    pub fn from_ms(val: f64) -> Self {
        Time::si(val / 1_000.0)
    }

    /// To milliseconds.
    pub fn to_ms(&self) -> f64 {
        self.val() * 1_000.0
    }

    /// From minutes.
    pub fn from_min(val: f64) -> Self {
        Time::si(val * 60.0)
    }

    /// To minutes.
    pub fn to_min(&self) -> f64 {
        self.val() / 60.0
    }

    /// From hours.
    pub fn from_hour(val: f64) -> Self {
        Time::si(val * 3600.0)
    }

    /// To hours.
    pub fn to_hour(&self) -> f64 {
        self.val() / 3600.0
    }

    /// From days.
    pub fn from_day(val: f64) -> Self {
        Time::si(val * 86400.0)
    }

    /// To days.
    pub fn to_day(&self) -> f64 {
        self.val() / 86400.0
    }

    /// From weeks.
    pub fn from_week(val: f64) -> Self {
        Time::si(val * 604800.0)
    }

    /// To weeks.
    pub fn to_week(&self) -> f64 {
        self.val() / 604800.0
    }

    /// From years.
    pub fn from_year(val: f64) -> Self {
        Time::si(val * 31536000.0)
    }

    /// To years.
    pub fn to_year(&self) -> f64 {
        self.val() / 31536000.0
    }

    /// From leap years.
    pub fn from_leapy(val: f64) -> Self {
        Time::si(val * 31622400.0)
    }

    /// To leap years.
    pub fn to_leapy(&self) -> f64 {
        self.val() / 31622400.0
    }
}