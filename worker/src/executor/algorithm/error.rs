use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum OpError {
  NonInvertible,
  NoCongruentDigit,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpError::NonInvertible => write!(f, "The Matrix can't be inverted"),
            OpError::NoCongruentDigit => write!(f, "A congruent digit was not found for the grid point")
            // OpError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            // OpError::ValidationError { field, message } => {
            //     write!(f, "Validation error on field '{}': {}", field, message)
            // }
        }
    }
}

impl Error for OpError {}