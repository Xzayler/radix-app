use std::fmt;

use nalgebra::{DMatrix, LpNorm, UniformNorm};

use crate::executor::algorithm::lib::spectral_norm;

pub trait Norm {
  fn get_matrix_norm(&self, matrix: &DMatrix<f64>) -> f64;
}

pub enum NormEnum {
  Infinite,
  L1,
  L2
}

impl fmt::Display for NormEnum {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Infinite => write!(f, "Infinite"),
      Self::L1 => write!(f, "L1"),
      Self::L2 => write!(f, "L2"),
    }
  }
}

impl Norm for NormEnum {
  fn get_matrix_norm(&self, matrix: &DMatrix<f64>) -> f64 {
    match self {
      NormEnum::Infinite => matrix.apply_norm(&UniformNorm),
      NormEnum::L1 => matrix.apply_norm(&LpNorm(1)),
      NormEnum::L2 => spectral_norm(&matrix)
    }
  }
}