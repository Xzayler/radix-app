use std::{error::Error, fmt};

use nalgebra::DVector;

#[derive(Debug)]
pub enum WorkerError {
  // User/input errors
  NonInvertibleBase,
  InvalidNorm { norm: String, message: String },
  InvalidInput(String),

  // Infra errors
  Environment(String),
  Database(String),
  Minio(String),

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
      Self::InvalidNorm { norm, message } => {
        write!(f, "Norm {norm} can't be applied to this system: {message}")
      }
      Self::InvalidInput(message) => write!(f, "Invalid input: {message}"),
      Self::Environment(message) => write!(f, "Config error: {message}"),
      Self::Database(message) => write!(f, "Database error: {message}"),
      Self::Minio(message) => write!(f, "S3 storage error: {message}"),
      Self::NoCongruentDigit(point) => write!(
        f,
        "A congruent digit was not found for the grid point {point}"
      ),
      Self::Operation(message) => write!(f, "Operation error: {message}"),
      Self::NoMatchingSystem => write!(f, "Could not choose a system model for this input"),
      Self::Unhandled(message) => write!(f, "Unexpected error: {message}"),
    }
  }
}

impl Error for WorkerError {}
