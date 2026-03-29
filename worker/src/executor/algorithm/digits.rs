use nalgebra::{DMatrix, DVector};
use std::{error::Error, fmt};

use crate::executor::algorithm::functions::{get_smith_values, get_vector_norm};
use crate::executor::algorithm::models::Norms;

#[derive(Debug)]
pub enum DigitsError {
  NonInvertibleBase,
  InvalidAxis { axis: usize, dimension: usize },
  InvalidShift { shift: u32, abs_det: i64 }
}

impl fmt::Display for DigitsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NonInvertibleBase => write!(f, "the base matrix is not invertible"),
      Self::InvalidAxis { axis, dimension } => {
        write!(f, "axis {axis} is out of bounds for dimension {dimension}")
      }
      Self::InvalidShift { shift, abs_det } => write!(
        f,
        "shift {shift} must be between 0 and abs(determinant) - 1 (= {})",
        abs_det - 1
      )
    }
  }
}

impl Error for DigitsError {}

pub fn explicit_digits(digits: Vec<DVector<f64>>)
-> impl Iterator<Item = DVector<f64>> {
  digits.into_iter()
}

pub fn canonical_digits(
  dim: usize,
  abs_determinant: u64
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  j_canonical_digits(dim, abs_determinant, 0)
}

pub fn canonical_digits_vec(
  dim: usize,
  abs_determinant: u64
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(canonical_digits(dim, abs_determinant)?.collect())
}

pub fn j_canonical_digits(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  validate_axis(dim, j_value)?;
  Ok(axis_digits(dim, abs_determinant, j_value, move |value| value as f64))
}

pub fn j_canonical_digits_vec(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(j_canonical_digits(dim, abs_determinant, j_value)?.collect())
}

pub fn symmetric_digits(
  dim: usize,
  abs_determinant: u64,
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  j_symmetric_digits(dim, abs_determinant, 0)
}

pub fn symmetric_digits_vec(
  dim: usize,
  abs_determinant: u64,
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(symmetric_digits(dim, abs_determinant)?.collect())
}

pub fn j_symmetric_digits(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  validate_axis(dim, j_value)?;
  let center = (abs_determinant / 2) as f64;
  Ok(axis_digits(dim, abs_determinant, j_value, move |value| {
    value as f64 - center
  }))
}

pub fn j_symmetric_digits_vec(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(j_symmetric_digits(dim, abs_determinant, j_value)?.collect())
}

pub fn shifted_canonical_digits(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
  shift: u32,
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  validate_axis(dim, j_value)?;
  validate_shift(abs_determinant, shift)?;
  Ok(axis_digits(dim, abs_determinant, j_value, move |value| {
    value as f64 - shift as f64
  }))
}

pub fn shifted_canonical_digits_vec(
  dim: usize,
  abs_determinant: u64,
  j_value: usize,
  shift: u32,
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(shifted_canonical_digits(dim, abs_determinant, j_value, shift)?.collect())
}

pub fn adjoint_digits(
  dim: usize,
  determinant: i64,
  base: &DMatrix<f64>
) -> Result<impl Iterator<Item = DVector<f64>>, DigitsError> {
  let abs_determinant = determinant.unsigned_abs();
  let (u, g_vec) = get_smith_data(base);
  let u_inv = u
    .try_inverse()
    .ok_or(DigitsError::NonInvertibleBase)?;
  let det = determinant as f64;
  let base_inv = base
    .clone()
    .try_inverse()
    .ok_or(DigitsError::NonInvertibleBase)?;
  let adjugate = base_inv * det;
  let zero = DVector::from_element(dim, 0.0);
  let base = base.clone();

  Ok(complete_residue_vectors(dim, abs_determinant, g_vec).map(move |residue| {
    let vector = &u_inv * residue;
    let rounded = round_vector(&vector);
    if rounded == zero {
      rounded
    } else {
      adjoint_congruent_element(dim, determinant, &base, &adjugate, &rounded)
    }
  }))
}

pub fn adjoint_digits_vec(
  dim: usize,
  determinant: i64,
  base: &DMatrix<f64>
) -> Result<Vec<DVector<f64>>, DigitsError> {
  Ok(adjoint_digits(dim, determinant, base)?.collect())
}

pub fn dense_digits_vec(
  dim: usize,
  determinant: i64,
  base: &DMatrix<f64>,
  norm: &Norms
) -> Result<Vec<DVector<f64>>, DigitsError> {
  let mut digits = adjoint_digits_vec(dim, determinant, base)?;
  let step = determinant.unsigned_abs() as f64;

  loop {
    let previous = digits.clone();
    let mut updated = Vec::with_capacity(previous.len());

    for digit in previous.iter() {
      let mut current = digit.clone();
      // Separate norm for vectors?
      let mut best_norm = get_vector_norm(&(base * &current), norm);

      for axis in 0..dim {
        current[axis] += step;
        let mut shifted = false;

        while get_vector_norm(&(base * &current), norm) < best_norm {
          current[axis] += step;
          best_norm = get_vector_norm(&(base * &current), norm);
          shifted = true;
        }

        if shifted {
          current[axis] -= step;
        } else {
          current[axis] -= 2.0 * step;
          while get_vector_norm(&(base * &current), norm) < best_norm {
            current[axis] -= step;
            best_norm = get_vector_norm(&(base * &current), norm);
          }
          current[axis] += step;
        }
      }

      updated.push(current);
    }

    if updated == digits {
      return Ok(updated);
    }

    digits = updated;
  }
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
  if shift < 0 || shift > (abs_determinant - 1) as u32 {
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

fn get_smith_data(m: &DMatrix<f64>) -> (DMatrix<f64>, Vec<i64>) {
  let (u, g) = get_smith_values(m);
  let diagonal = (0..g.ncols())
    .map(|index| g[(index, index)].round() as i64)
    .collect();

  (u, diagonal)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn canonical_digits_test() {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, 0.0, 0.0, 2.0]);
    let dim = base.ncols();
    let abs_determinant = (base.determinant() as i64).unsigned_abs();
    let digits = canonical_digits_vec(
      dim,
      abs_determinant
    ).expect("valid axis");

    assert_eq!(digits.len(), 4);
    assert_eq!(digits[0], DVector::from_vec(vec![0.0, 0.0]));
    assert_eq!(digits[1], DVector::from_vec(vec![1.0, 0.0]));
    assert_eq!(digits[2], DVector::from_vec(vec![2.0, 0.0]));
    assert_eq!(digits[3], DVector::from_vec(vec![3.0, 0.0]));
  }

  #[test]
  fn j_canonical_digits_test() {
    let base = DMatrix::from_row_slice(3, 3, &[2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0]);
    let dim = base.ncols();
    let abs_determinant = (base.determinant() as i64).unsigned_abs();
    let digits = j_canonical_digits_vec(dim, abs_determinant, 2).expect("valid axis");

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
    let det = 4;
    let digits = symmetric_digits_vec(2, det).expect("valid axis");
    println!("Symm digits: {:?}", digits);

    assert_eq!(digits.len(), det as usize);
    assert_eq!(digits[0], DVector::from_vec(vec![-2.0, 0.0]));
    assert_eq!(digits[1], DVector::from_vec(vec![-1.0, 0.0]));
    assert_eq!(digits[2], DVector::from_vec(vec![0.0, 0.0]));
    assert_eq!(digits[3], DVector::from_vec(vec![1.0, 0.0]));
  }

  #[test]
  fn shifted_digits_validate_shift() {
    let base = DMatrix::from_row_slice(1, 1, &[3.0]);
    let dim = base.ncols();
    let abs_determinant = (base.determinant() as i64).unsigned_abs();
    let error = shifted_canonical_digits_vec(dim, abs_determinant, 0, 3).unwrap_err();

    assert!(matches!(error, DigitsError::InvalidShift { .. }));
  }

  #[test]
  fn adjoint_digits_test() {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let dim = base.ncols();
    let digits: Vec<_> = adjoint_digits(dim, base.determinant() as i64, &base)
      .expect("generated digits")
      .collect();
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

 #[test]
  fn dense_digits_test() {
    let base = DMatrix::from_row_slice(4, 4, &[
      1.0, -1.0, 0.0, 0.0,
      1.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 2.0, -1.0,
      0.0, 0.0, 1.0, 2.0
      ]);
    let dim = base.ncols();
    let expected = vec![
      DVector::from_row_slice(&[0.0, 0.0, 0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0, 1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0, 0.0, 1.0]),
      DVector::from_row_slice(&[1.0, 1.0, 1.0, 1.0]),
      DVector::from_row_slice(&[-1.0, 0.0, -1.0, 0.0]),
      DVector::from_row_slice(&[0.0, -1.0, 0.0, -1.0]),
      DVector::from_row_slice(&[-1.0, -1.0, -1.0, -1.0]),
      DVector::from_row_slice(&[-1.0, 1.0, -1.0, 1.0]),
      DVector::from_row_slice(&[1.0, -1.0, 1.0, -1.0]),
      DVector::from_row_slice(&[-1.0, 2.0, -1.0, 2.0])
    ];
    let digits = dense_digits_vec(dim, base.determinant() as i64, &base, &Norms::Infinite).expect("generated digist");
    println!("{:?}", digits);
    assert_eq!(digits.len(), expected.len());
    assert!(expected.iter().all(|expected_digit| digits.contains(expected_digit)));
  }

}
