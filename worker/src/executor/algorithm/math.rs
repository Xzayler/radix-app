use crate::{
  error::WorkerError,
  executor::algorithm::{
    digits::{SystemDigits, SystemDigitsEnum},
    norms::{Norm, NormEnum},
  },
};
use rayon::prelude::*;
use std::collections::HashMap;

extern crate nalgebra as na;
use algebraeon::nzq::Integer;
use algebraeon::rings::matrix::Matrix as Alg_Matrix;
use na::{DMatrix, DVector};

fn get_smith_values(m: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
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
  (um, gm)
}

pub fn get_smith_data(m: &DMatrix<f64>) -> (DMatrix<f64>, Vec<i64>) {
  let (u, g) = get_smith_values(m);
  let diagonal = (0..g.ncols())
    .map(|index| g[(index, index)].round() as i64)
    .collect();

  (u, diagonal)
}

pub fn hash_point(
  dim: usize,
  s: usize,
  u: &DMatrix<f64>,
  g: &Vec<i64>,
  g_prods: &Vec<i64>,
  point: &DVector<f64>,
) -> i64 {
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

pub fn build_h_i(
  dim: usize,
  s: usize,
  u: &DMatrix<f64>,
  g: &Vec<i64>,
  g_prods: &Vec<i64>,
  digits: &SystemDigitsEnum,
) -> HashMap<i64, DVector<f64>> {
  digits
    .get_digits_iter()
    .par_bridge()
    .map(|digit| (hash_point(dim, s, u, g, g_prods, &digit), digit))
    .collect()
}

pub fn spectral_norm(m: &DMatrix<f64>) -> f64 {
  let prod = m.transpose() * m;
  let svd = prod.svd(false, false);
  svd.singular_values[0].sqrt()
}

pub fn find_c_gamma(m_inv: &DMatrix<f64>, norm: &NormEnum) -> Result<(usize, f64), WorkerError> {
  let norm_threshold: f64 = 0.01;
  let mut c: usize = 1;
  let inv_norm = norm.get_matrix_norm(&m_inv);
  if inv_norm >= 1.0 {
    return Err(WorkerError::InvalidNorm {
      norm: norm.to_string(),
      message: "Base inverse not contractive".to_string(),
    });
  }

  let mut m_pow = m_inv.clone();

  while norm.get_matrix_norm(&m_pow) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - norm.get_matrix_norm(&m_pow));

  Ok((c, gamma))
}

pub fn satisfies_unit_condition(base: &DMatrix<f64>) -> bool {
  let dim: usize = base.ncols();
  let identity_matrix: DMatrix<f64> = DMatrix::identity(dim, dim);
  let det = (identity_matrix - base).determinant().round();
  det != 1.0 && det != -1.0
}
