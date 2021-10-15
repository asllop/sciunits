use std::ops::Deref;
use crate::magnitude::Magnitude;

#[macro_export]
macro_rules! array {
    ( $( $x:expr ),* ) => {
        {
            Array::new(vec!($( $x ),*))
        }
    };
}

/// Array of magnitudes.
#[derive(Clone)]
pub struct Array<M: Magnitude>(Vec<M>);

impl<M: Magnitude> Array<M> {
    pub fn new(v: Vec<M>) -> Self {
        Self(v)
    }
}

impl<M: Magnitude> std::fmt::Display for Array<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.iter().enumerate() {
            if i == self.len() - 1 {
                write!(f, " {} ", x)?;
            }
            else {
                write!(f, " {} ,", x)?;
            }
        }
        write!(f, "]")
    }
}

impl<M: Magnitude> Deref for Array<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}