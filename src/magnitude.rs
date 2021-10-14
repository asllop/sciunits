
#[macro_export]
macro_rules! impl_magnitude_generics {
    ( $x:ty ) => {
        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.val(), self.units())
            }
        }

        impl std::ops::Div for $x {
            type Output = f64;
        
            fn div(self, rhs: $x) -> Self::Output {
                self.val() / rhs.val()
            }
        }

        //TODO: afegeix exponent a les unitats, així en cas de multiplicar unitats, podem fer s^2, o m^3, etc.
        impl std::ops::Mul for $x {
            type Output = f64;
        
            fn mul(self, rhs: $x) -> Self::Output {
                self.val() / rhs.val()
            }
        }

        impl std::ops::Add for $x {
            type Output = $x;
        
            fn add(self, rhs: $x) -> Self::Output {
                <$x>::si(self.val() + rhs.val())
            }
        }

        impl std::ops::Sub for $x {
            type Output = $x;
        
            fn sub(self, rhs: $x) -> Self::Output {
                <$x>::si(self.val() - rhs.val())
            }
        }
    };
}

use std::ops::{Add, Sub, Mul, Div};

/// Trait to define types that contain magnitudes of specified units.
pub trait Magnitude : Add + Sub + Mul + Div + Sized {
    /// Build a new magnitude using SI units.
    fn si(val: f64) -> Self where Self: Sized;
    /// Return magnitude in SI units.
    fn val(&self) -> f64;
    /// Units symbol of current magnitude in SI.
    fn units(&self) -> String;
}
