use crate::executor::algorithm::{digits::{SystemDigits, SystemDigitsEnum}, models::{Norms, WorkerError}};
use std::collections::HashMap;

extern crate nalgebra as na;
use algebraeon::nzq::Integer;
use algebraeon::rings::matrix::Matrix as Alg_Matrix;
use na::{DMatrix, DVector};
use nalgebra::{EuclideanNorm, LpNorm, Norm, UniformNorm};

// TODO: Big work: Generally use i64 instead of floats
fn get_smith_values(m: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
  // Assuming m is a square matrix
  let dim = m.ncols();
  let cols: Vec<Vec<Integer>> = m
    .column_iter()
    .map(|col| {
      col.into_iter()
        .map(|elem| Integer::from(elem.clone() as i32))
        .collect()
    })
    .collect();
  let alg_mat: Alg_Matrix<Integer> = Alg_Matrix::from_cols(cols);
  let (u, g, _v, _k) = alg_mat.smith_algorithm();
  let um: DMatrix<f64> = DMatrix::from_row_slice(
    dim,
    dim,
    &(u.entries_list()
      .into_iter()
      .map(|i| i.into())
      .collect::<Vec<f64>>()),
  );
  let gm: DMatrix<f64> = DMatrix::from_row_slice(
    dim,
    dim,
    &(g.entries_list()
      .into_iter()
      .map(|i| i.into())
      .collect::<Vec<f64>>()),
  );
  return (um, gm);
}

pub fn get_smith_data(m: &DMatrix<f64>) -> (DMatrix<f64>, Vec<i64>) {
  let (u, g) = get_smith_values(m);
  let diagonal = (0..g.ncols())
    .map(|index| g[(index, index)].round() as i64)
    .collect();

  (u, diagonal)
}

pub fn hash_point(dim: usize, s: usize, u: &DMatrix<f64>, g: &Vec<i64>, g_prods: &Vec<i64>, point: &DVector<f64>) -> i64 {
  let uz = u * point;

  let mut h = 0;
  for i in s..dim {
    let gi = g[i];
    let mut r = (uz[(i, 0)] as i64) % gi;
    if r < 0 {
      r += gi;
    }
    h += g_prods[i] * r; // TODO: i-1?
  }
  h
}

pub fn build_h_i<'a>(
  dim: usize,
  s: usize,
  u: &DMatrix<f64>,
  g: &Vec<i64>,
  g_prods: &Vec<i64>,
  digits: &SystemDigitsEnum,
) -> HashMap<i64, DVector<f64>> {
  let mut h_map: HashMap<i64, DVector<f64>> = HashMap::new();
  let iter = digits.get_digits_iter();

  for digit in iter {
    let key = hash_point(dim, s, u, g, g_prods, &digit);
    h_map.insert(key, digit);
  }
  h_map
}

pub fn get_vector_norm(v: &DVector<f64>, norm: &Norms) -> f64 {
  match norm {
    Norms::Infinite => v.apply_norm(&UniformNorm),
    Norms::L1 => v.apply_norm(&LpNorm(1)),
    Norms::L2 => v.apply_norm(&EuclideanNorm)
  }
}

fn spectral_norm(m: &DMatrix<f64>) -> f64 {
  let prod = m.transpose() * m;
  let svd = prod.svd(false, false);
  svd.singular_values[0].sqrt()
}

pub fn get_matrix_norm(m: &DMatrix<f64>, norm: &Norms) -> f64 {
  match norm {
    Norms::Infinite => m.apply_norm(&UniformNorm),
    Norms::L1 => m.apply_norm(&LpNorm(1)),
    Norms::L2 => spectral_norm(m)
  }
}

fn find_c_gamma_spectral(m_inv: &DMatrix<f64>) -> Result<(usize, f64), WorkerError> {
  let norm_threshold: f64 = 0.01;
  let mut c: usize = 1;
  let inv_norm = spectral_norm(&m_inv);
  if inv_norm >= 1.0 {
    return Err(WorkerError::InvalidNorm(Norms::L2));
  }

  let mut m_pow = m_inv.clone();

  while spectral_norm(&m_pow) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - spectral_norm(&m_pow));

  Ok((c, gamma))
}

fn find_c_gamma_norm(m_inv: &DMatrix<f64>, norm: &impl Norm<f64>, norm_type: Norms) -> Result<(usize, f64), WorkerError> {
  let norm_threshold: f64 = 0.01;
  let mut c: usize = 1;
  let inv_norm = m_inv.apply_norm(norm);
  if inv_norm >= 1.0 {
    return Err(WorkerError::InvalidNorm(norm_type));
  }

  let mut m_pow = m_inv.clone();
  while m_pow.apply_norm(norm) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - m_pow.apply_norm(norm));

  Ok((c, gamma))
}

pub fn find_c_gamma(m_inv: &DMatrix<f64>, norm: &Norms) -> Result<(usize, f64), WorkerError> {
  match norm {
    Norms::L2 => find_c_gamma_spectral(m_inv),
    Norms::Infinite => find_c_gamma_norm(m_inv, &UniformNorm, Norms::Infinite),
    Norms::L1 => find_c_gamma_norm(m_inv, &LpNorm(1), Norms::L1),
  }
}

// pub fn get_cover_box(
//   m_inv: &DMatrix<f64>,
//   c: usize,
//   gamma: f64,
//   digits: &SystemDigitsEnum,
// ) -> Result<(Vec<i32>, Vec<i32>), WorkerError> {
//   let mut m_pow = m_inv.clone();
//   let dim = m_inv.ncols();

//   let mut sum_xi: DVector<f64> = DVector::from_element(dim, 0.0);
//   let mut sum_eta: DVector<f64> = DVector::from_element(dim, 0.0);

//   for j in 0..c {
//     let mut iter = digits.get_digits_iter();
//     let first = iter.next().ok_or(WorkerError::Unhandled("No digits found".to_string()))?;

//     let mut xi_j = &m_pow * &first;
//     let mut eta_j = xi_j.clone();

//     for digit in iter {
//       let prod = &m_pow * &digit;
//       for m in 0..dim {
//         if prod[m] > xi_j[m] {
//           xi_j[m] = prod[m];
//         }
//         if prod[m] < eta_j[m] {
//           eta_j[m] = prod[m];
//         }
//       }
//     }

//     sum_xi += xi_j;
//     sum_eta += eta_j;

//     if j < c - 1 {
//       m_pow = &m_pow * m_inv;
//     }
//   }


//   // println!("Sum_xi: {:?}", sum_xi);
//   // println!("Sum_eta: {:?}", sum_eta);

//   let mut l: Vec<i32> = vec![0; dim];
//   let mut u: Vec<i32> = vec![0; dim];
//   for m in 0..dim {
//     l[m] = (-1.0 * (&gamma * &sum_eta[m])).ceil() as i32;
//     u[m] = (-1.0 * (&gamma * &sum_xi[m])).floor() as i32;
//   }

//   Ok((u, l))
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::executor::algorithm::digits::{get_explicit};

// }
