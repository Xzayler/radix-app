use std::fmt::{self};
use std::error::Error;

#[derive(Debug)]
pub enum Norms {
  Spectral,
  Uniform,
  L1
}

#[derive(Debug)]
pub enum OpError {
  NonInvertible,
  NoCongruentDigit,
  InvalidNorm(Norms)
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpError::NonInvertible => write!(f, "The Matrix can't be inverted"),
            OpError::NoCongruentDigit => write!(f, "A congruent digit was not found for the grid point"),
            OpError::InvalidNorm(norm) => write!(f, "The norm {:?} cannot be used on this matrix", norm)
        }
    }
}

impl Error for OpError {}