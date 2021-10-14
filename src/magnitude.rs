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
    /// Build a new magnitude using SI units.
    fn si(val: f64) -> Self where Self: Sized;
    /// Magnitude in SI units.
    fn val(&self) -> f64;
    /// SI units of current magnitude.
    fn units(&self) -> String;
}
