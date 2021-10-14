use std::ops::{Add, Sub, Mul, Div};

#[macro_export]
macro_rules! impl_magnitude_generics {
    ( $x:ty ) => {
        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.val(), self.units())
            }
        }

        impl std::ops::Mul for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn mul(self, rhs: crate::array::Array<$x>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() * rhs[0].val();
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Mul<Vec<f64>> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn mul(self, rhs: Vec<f64>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() * rhs[0];
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Mul<f64> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn mul(self, rhs: f64) -> Self::Output {
                let sz = self.len();
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() * rhs;
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Div for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn div(self, rhs: crate::array::Array<$x>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() / rhs[0].val();
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Div<Vec<f64>> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn div(self, rhs: Vec<f64>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() / rhs[0];
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Div<f64> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn div(self, rhs: f64) -> Self::Output {
                let sz = self.len();
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() / rhs;
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Add for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn add(self, rhs: crate::array::Array<$x>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() + rhs[0].val();
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Add<Vec<f64>> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn add(self, rhs: Vec<f64>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() + rhs[0];
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Add<f64> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn add(self, rhs: f64) -> Self::Output {
                let sz = self.len();
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() + rhs;
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Sub for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn sub(self, rhs: crate::array::Array<$x>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() - rhs[0].val();
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Sub<Vec<f64>> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn sub(self, rhs: Vec<f64>) -> Self::Output {
                let sz = usize::min(self.len(), rhs.len());
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() - rhs[0];
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Sub<f64> for crate::array::Array<$x> {
            type Output = crate::array::Array<$x>;
        
            fn sub(self, rhs: f64) -> Self::Output {
                let sz = self.len();
                let mut v = Vec::new();
                for i in 0..sz {
                    let m = self[i].val() - rhs;
                    v.push(<$x>::si(m))
                }
                crate::array::Array::new(v)
            }
        }

        impl std::ops::Div for $x {
            type Output = $x;
        
            fn div(self, rhs: $x) -> Self::Output {
                <$x>::si(self.val() / rhs.val())
            }
        }

        impl std::ops::Mul for $x {
            type Output = $x;
        
            fn mul(self, rhs: $x) -> Self::Output {
                <$x>::si(self.val() / rhs.val())
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

/// Trait to define types that contain magnitudes of specified units.
pub trait Magnitude : Add + Sub + Mul + Div + Sized {
    /// Build a new magnitude using SI units.
    fn si(val: f64) -> Self where Self: Sized;
    /// Return magnitude in SI units.
    fn val(&self) -> f64;
    /// Units symbol of current magnitude in SI.
    fn units(&self) -> String;
}

/*
TODO:

Afegeix exponent a les unitats, així en cas de multiplicar unitats, podem fer s^2, o m^3, etc.
Podem fer-ho mitjançant un tipus Poly que conté un crate::array::Array de Magnitudes, cada pos de l'crate::array::Array és un grau del polinòmi.
Per exemple:
5s + 15s^2 + s^3 -> [0, 5, 15, 1]

Pot contenir un altre crate::array::Array per als exponents negatius, la primera pos de l'crate::array::Array sería s^-1, la segona, s^-2, etc.

Les operacions Mul i Div retornen un Poly.

Els Poly tenen una artmètica pròpia, amb operacions + - / * que retornen altres Poly.

*/