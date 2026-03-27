use std::error::Error;
use std::fmt::{self};

use nalgebra::DVector;

#[derive(Debug)]
pub enum Norms {
    Infinite,
    L2,
    L1,
}

#[derive(Debug)]
pub enum OpError {
    NonInvertible,
    NoCongruentDigit(DVector<f64>, i32),
    InvalidNorm(Norms),
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpError::NonInvertible => write!(f, "The Matrix can't be inverted"),
            OpError::NoCongruentDigit(point, h) => write!(
                f,
                "A congruent digit for was not found for the grid point {:?} with hash {:?}",
                point, h
            ),
            OpError::InvalidNorm(norm) => {
                write!(f, "The norm {:?} cannot be used on this matrix", norm)
            }
        }
    }
}

impl Error for OpError {}
