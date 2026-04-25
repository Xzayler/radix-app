use nalgebra::{DMatrix, DVector};
use std::{error::Error, fmt};

use crate::executor::algorithm::lib::get_smith_data;

const BASE_NAME: &str = "base";
const U_NAME: &str = "base's smith component";

#[derive(Debug)]
pub enum DigitsError {
  InvalidExplicitDigitCount { expected: usize, actual: usize},
  NonInvertible(String),
  InvalidAxis { axis: usize, dimension: usize },
  InvalidShift { shift: u32, abs_det: i64 }
}

impl fmt::Display for DigitsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidExplicitDigitCount {expected, actual} =>
        write!(f, "Explicit digit count is incorrect. Expected {expected}, got {actual}."),
      Self::NonInvertible(subject) => 
        write!(f, "The {subject} matrix is not invertible"),
      Self::InvalidAxis { axis, dimension } =>
        write!(f, "Axis {axis} is out of bounds for dimension {dimension}"),
      Self::InvalidShift { shift, abs_det } => write!(
        f,
        "Shift {shift} must be between 0 and abs(determinant) - 1 (= {})",
        abs_det - 1
        )
    }
  }
}

impl Error for DigitsError {}

pub enum SystemDigitsEnum {
  Explicit(ExplicitDigits),
  Canonical(CanonicalDigits),
  Symmetric(SymmetricDigits),
  Shifted(ShiftedCanonicalDigits),
  Adjoint(AdjointDigits)
}

impl SystemDigits for SystemDigitsEnum {
  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    match self {
      SystemDigitsEnum::Explicit(d) => d.get_digits_vec(),
      SystemDigitsEnum::Canonical(d) => d.get_digits_vec(),
      SystemDigitsEnum::Symmetric(d) => d.get_digits_vec(),
      SystemDigitsEnum::Shifted(d) => d.get_digits_vec(),
      SystemDigitsEnum::Adjoint(d) => d.get_digits_vec()
    }
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    match self {
      SystemDigitsEnum::Explicit(d) => d.get_digits_iter(),
      SystemDigitsEnum::Canonical(d) => d.get_digits_iter(),
      SystemDigitsEnum::Symmetric(d) => d.get_digits_iter(),
      SystemDigitsEnum::Shifted(d) => d.get_digits_iter(),
      SystemDigitsEnum::Adjoint(d) => d.get_digits_iter()
    }
  }
}

pub trait SystemDigits {
  fn get_digits_vec(&self) -> Vec<DVector<f64>>;
  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_>;
}

pub struct ExplicitDigits {
  digits: Vec<DVector<f64>>,
}

impl SystemDigits for ExplicitDigits {
  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    self.digits.clone()
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    Box::new(self.digits.clone().into_iter())
  }
}

pub fn get_explicit(base: &DMatrix<f64>, digits: Vec<DVector<f64>>) -> Result<ExplicitDigits, DigitsError> {
  let det = base.determinant() as i64;
  let abs_det = det.unsigned_abs() as usize;
  if digits.len() != abs_det {
    return Err(DigitsError::InvalidExplicitDigitCount { expected: abs_det, actual: digits.len() });
  }
  Ok(ExplicitDigits { digits })
}

pub struct CanonicalDigits {
  dim: usize,
  abs_det: u64,
  j_value: usize,
}

impl SystemDigits for CanonicalDigits {
  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    axis_digits(self.dim, self.abs_det, self.j_value, |value| value as f64).collect()
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    Box::new(axis_digits(self.dim, self.abs_det, self.j_value, |value| value as f64))
  }
}

pub fn get_canonical(base: &DMatrix<f64>) -> Result<CanonicalDigits, DigitsError> {
  get_j_canonical(base, 0)
}

pub fn get_j_canonical(base: &DMatrix<f64>, j_value: usize) -> Result<CanonicalDigits, DigitsError> {
  let dim = base.ncols();
  validate_axis(dim, j_value)?;
  let abs_det = (base.determinant() as i64).unsigned_abs();
  Ok(CanonicalDigits { dim, abs_det, j_value })
}

pub struct SymmetricDigits {
  dim: usize,
  abs_det: u64,
  j_value: usize,
}

impl SystemDigits for SymmetricDigits {
  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    let center = (self.abs_det / 2) as f64;
    axis_digits(self.dim, self.abs_det, self.j_value, move |value| value as f64 - center).collect()
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    let center = (self.abs_det / 2) as f64;
    Box::new(axis_digits(self.dim, self.abs_det, self.j_value, move |value| value as f64 - center))
  }
}

pub fn get_symmetric(base: &DMatrix<f64>) -> Result<SymmetricDigits, DigitsError> {
  get_j_symmetric(base, 0)
}

pub fn get_j_symmetric(base: &DMatrix<f64>, j_value: usize) -> Result<SymmetricDigits, DigitsError> {
  let dim = base.ncols();
  validate_axis(dim, j_value)?;
  let abs_det = (base.determinant() as i64).unsigned_abs();
  Ok(SymmetricDigits { dim, abs_det, j_value })
}

pub struct ShiftedCanonicalDigits {
  dim: usize,
  abs_det: u64,
  j_value: usize,
  shift: u32,
}

impl SystemDigits for ShiftedCanonicalDigits {
  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    let shift = self.shift;
    axis_digits(self.dim, self.abs_det, self.j_value, move |value| value as f64 - shift as f64).collect()
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    let shift = self.shift;
    Box::new(axis_digits(self.dim, self.abs_det, self.j_value, move |value| value as f64 - shift as f64))
  }
}

pub fn get_shifted_canonical(base: &DMatrix<f64>, j_value: usize, shift: u32) -> Result<ShiftedCanonicalDigits, DigitsError> {
  let dim = base.ncols();
  validate_axis(dim, j_value)?;
  let abs_det = (base.determinant() as i64).unsigned_abs();
  validate_shift(abs_det, shift)?;
  Ok(ShiftedCanonicalDigits { dim, abs_det, j_value, shift })
}

pub struct AdjointDigits {
  dim: usize,
  g_vec: Vec<i64>,
  det: i64,
  abs_det: u64,
  base: DMatrix<f64>,
  base_inv: DMatrix<f64>,
  u_inv: DMatrix<f64>
}

impl SystemDigits for AdjointDigits {

  fn get_digits_vec(&self) -> Vec<DVector<f64>> {
    let abs_determinant = self.abs_det;
    let det = self.det as f64;
    let adjugate = &self.base_inv * det;
    let zero = DVector::from_element(self.dim, 0.0);

    complete_residue_vectors(self.dim, abs_determinant, self.g_vec.clone()).map(move |residue| {
      let vector = &self.u_inv * residue;
      let rounded = round_vector(&vector);
      if rounded == zero {
        rounded
      } else {
        adjoint_congruent_element(self.dim, self.det, &self.base, &adjugate, &rounded)
      }
    }).collect()
  }

  fn get_digits_iter(&self) -> Box<dyn Iterator<Item = DVector<f64>> + Send + '_> {
    let abs_determinant = self.abs_det;
    let det = self.det as f64;
    let adjugate = &self.base_inv * det;
    let zero = DVector::from_element(self.dim, 0.0);

    Box::new(complete_residue_vectors(self.dim, abs_determinant, self.g_vec.clone()).map(move |residue| {
      let vector = &self.u_inv * residue;
      let rounded = round_vector(&vector);
      if rounded == zero {
        rounded
      } else {
        adjoint_congruent_element(self.dim, self.det, &self.base, &adjugate, &rounded)
      }
    }))
  }
}

pub fn get_adjoint(base: &DMatrix<f64>) -> Result<AdjointDigits, DigitsError> {
  let base_inv = match base.clone().try_inverse() {
    Some(inv) => inv,
    None => return Err(DigitsError::NonInvertible(BASE_NAME.to_string()))
  };

  let (u, g_vec) = get_smith_data(&base);
  let u_inv = match u.clone().try_inverse() {
    Some(inv) => inv,
    // TODO: New Error
    None => return Err(DigitsError::NonInvertible(U_NAME.to_string()))
  };
  let det = base.determinant() as i64;
  Ok(AdjointDigits { dim: base.ncols(), det, abs_det: det.unsigned_abs(), base: base.clone(), base_inv, u_inv, g_vec })
}

fn axis_digits(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
  value_fn: impl Fn(u64) -> f64 + 'static,
) -> impl Iterator<Item = DVector<f64>> {
  (0..abs_determinant).map(move |value| {
    let mut digit = DVector::from_element(dim, 0.0);
    digit[j_value] = value_fn(value);
    digit
  })
}

fn validate_axis(dim: usize, axis: usize) -> Result<(), DigitsError> {
  if axis < dim {
    Ok(())
  } else {
    Err(DigitsError::InvalidAxis {
      axis,
      dimension: dim,
    })
  }
}

fn validate_shift(abs_determinant: u64, shift: u32) -> Result<(), DigitsError> {
  if shift as u64 > (abs_determinant - 1) {
    Err(DigitsError::InvalidShift {
      shift,
      abs_det: abs_determinant as i64,
    })
  } else {
    Ok(())
  }
}

fn complete_residue_vectors(dim: usize, abs_determinant: u64, g: Vec<i64>) -> impl Iterator<Item = DVector<f64>> {
  let bases = g.to_vec();

  (0..abs_determinant).map(move |mut index| {
    let mut coords = vec![0.0; dim];

    for axis in (0..dim).rev() {
      let radix = bases[axis].max(1) as u64;
      coords[axis] = (index % radix) as f64;
      index /= radix;
    }

    DVector::from_vec(coords)
  })
}

fn adjoint_congruent_element(
  dim: usize,
  determinant: i64,
  base: &DMatrix<f64>,
  adjugate: &DMatrix<f64>,
  point: &DVector<f64>,
) -> DVector<f64> {
  let det = determinant as f64;
  let mut reduced = DVector::from_element(dim, 0.0);
  for row in 0..dim {
    let component: f64 = (0..dim)
      .map(|col| adjugate[(row, col)] * point[col])
      .sum();
    reduced[row] = symmetric_modulo(component.round() as i64, determinant) as f64;
  }

  round_vector(&((base * reduced) / det))
}

fn symmetric_modulo(value: i64, modulus: i64) -> i64 {
  let modulus = modulus.abs();
  let mut remainder = value % modulus;
  if remainder < 0 {
    remainder += modulus;
  }
  if remainder > modulus / 2 {
    remainder - modulus
  } else {
    remainder
  }
}

fn round_vector(vector: &DVector<f64>) -> DVector<f64> {
  vector.map(|value| value.round())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn canonical_digits_test() {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, 0.0, 0.0, 2.0]);
    let canonical = get_canonical(&base).expect("valid axis");
    let digits = canonical.get_digits_vec();

    assert_eq!(digits.len(), 4);
    assert_eq!(digits[0], DVector::from_vec(vec![0.0, 0.0]));
    assert_eq!(digits[1], DVector::from_vec(vec![1.0, 0.0]));
    assert_eq!(digits[2], DVector::from_vec(vec![2.0, 0.0]));
    assert_eq!(digits[3], DVector::from_vec(vec![3.0, 0.0]));
  }

  #[test]
  fn j_canonical_digits_test() {
    let base = DMatrix::from_row_slice(3, 3, &[2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0]);
    let j_canonical = get_j_canonical(&base, 2).expect("ok");
    let digits = j_canonical.get_digits_vec();

    assert_eq!(digits.len(), 8);
    assert_eq!(digits[0], DVector::from_vec(vec![0.0, 0.0, 0.0]));
    assert_eq!(digits[1], DVector::from_vec(vec![0.0, 0.0, 1.0]));
    assert_eq!(digits[2], DVector::from_vec(vec![0.0, 0.0, 2.0]));
    assert_eq!(digits[3], DVector::from_vec(vec![0.0, 0.0, 3.0]));
    assert_eq!(digits[4], DVector::from_vec(vec![0.0, 0.0, 4.0]));
    assert_eq!(digits[5], DVector::from_vec(vec![0.0, 0.0, 5.0]));
    assert_eq!(digits[6], DVector::from_vec(vec![0.0, 0.0, 6.0]));
    assert_eq!(digits[7], DVector::from_vec(vec![0.0, 0.0, 7.0]));
  }

  #[test]
  fn symmetric_digits_test() {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, 0.0, 0.0, 2.0]);
    let det = (base.determinant() as i64).unsigned_abs();
    let symmetric = get_symmetric(&base).expect("ok");
    let digits = symmetric.get_digits_vec();

    assert_eq!(digits.len(), det as usize);
    assert_eq!(digits[0], DVector::from_vec(vec![-2.0, 0.0]));
    assert_eq!(digits[1], DVector::from_vec(vec![-1.0, 0.0]));
    assert_eq!(digits[2], DVector::from_vec(vec![0.0, 0.0]));
    assert_eq!(digits[3], DVector::from_vec(vec![1.0, 0.0]));
  }

  #[test]
  fn shifted_digits_validate_shift() {
    let base = DMatrix::from_row_slice(1, 1, &[3.0]);
    let shifted = get_shifted_canonical(&base, 0, 3);
    match shifted {
      Ok(_res) => panic!(),
      Err(err) => {
      assert!(matches!(err, DigitsError::InvalidShift { .. }));
      }
    };
  }

  #[test]
  fn adjoint_digits_test() {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let adjoint = get_adjoint(&base).unwrap();
    let digits = adjoint.get_digits_vec();
    let expected = vec![
      DVector::from_vec(vec![0.0, 0.0]),
      DVector::from_vec(vec![0.0, 1.0]),
      DVector::from_vec(vec![1.0, 0.0]),
      DVector::from_vec(vec![-1.0, 0.0]),
      DVector::from_vec(vec![0.0, -1.0])
    ];
    assert_eq!(digits.len(), expected.len());
    assert!(expected.iter().all(|expected_digit| digits.contains(expected_digit)));
  }
}
