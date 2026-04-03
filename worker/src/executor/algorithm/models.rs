use std::{error::Error, fmt::{self, Formatter}};

use nalgebra::DVector;

#[derive(Debug, Clone)]
pub enum Norms {
  Infinite,
  L2,
  L1,
}

impl fmt::Display for Norms {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Infinite => write!(f, "Infinite"),
      Self::L1 => write!(f, "L1"),
      Self::L2 => write!(f, "L2")
    }
  }
}

#[derive(Debug)]
pub enum WorkerError {
  // User/input errors
  NonInvertibleBase,
  InvalidNorm(Norms),
  InvalidInput(String),
  
  // Infra errors
  Database(String),

  // Program errors
  NoCongruentDigit(DVector<f64>),
  Operation(String),
  NoMatchingSystem,
  Unhandled(String),
}

impl fmt::Display for WorkerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NonInvertibleBase => write!(f, "The base is not invertible"),
      Self::InvalidNorm(norm) => write!(f, "Norm {} can't be applied to this system", norm),
      Self::InvalidInput(message) => write!(f, "Invalid input: {}", message),
      Self::Database(message) => write!(f, "Database error: {}", message),
      Self::NoCongruentDigit(point) => write!(
                f,
                "A congruent digits was not found for the grid point {:?}",
                point
            ),
      Self::Operation(message) => write!(f, "Operation error: {}", message),
      Self::NoMatchingSystem => write!(f, "Could choose a system model for this input"),
      Self::Unhandled(message) => write!(f, "Unexpected error: {}", message),
    }
  }
}

impl Error for WorkerError {}
