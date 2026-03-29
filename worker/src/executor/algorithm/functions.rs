use crate::executor::algorithm::{digits::{SystemDigits, SystemDigitsEnum, get_explicit}, models::{Norms, OpError}};
use std::{collections::HashMap, vec};

extern crate nalgebra as na;
use algebraeon::nzq::Integer;
use algebraeon::rings::matrix::Matrix as Alg_Matrix;
use na::{DMatrix, DVector};
use nalgebra::{EuclideanNorm, LpNorm, Norm, UniformNorm};

pub struct PreComputed<T: Clone> {
  pub m_inv: DMatrix<T>,
  pub g: DMatrix<T>, // maybe just Vec<T>?
  pub u: DMatrix<T>,
  // grid_points: Vec<Vec<T>>
  // _i: Vec<u16> //precalculated h values for all digits?
}


// TODO: Big work: Generally use i64 instead of floats
pub(crate) fn get_smith_values(m: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
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

pub fn pre_compute<'a>(
  m: DMatrix<f64>
) -> Result<PreComputed<f64>, OpError> {
  let (u, g) = get_smith_values(&m);

  let m_inv = match m.try_inverse() {
    Some(inv) => inv,
    None => return Err(OpError::NonInvertible),
  };

  let p = PreComputed {
    m_inv: m_inv,
    g: g,
    u: u,
  };

  Ok(p)
}

// TODO: Set to provate
pub fn hash_point(dim: usize, u: &DMatrix<f64>, g: &Vec<i64>, point: &DVector<f64>) -> i64 {
  let uz = u * point;

  let n = dim;
  let mut s = 0; // TODO: precompute s?
  while s < n && g[s] == 1 {
    s += 1;
  }

  let mut h = 0;
  for i in s..n {
    let mut prod: i64 = 1;
    for j in s..i {
      prod *= g[j] as i64; // TODO: Precompute these products
    }
    let gi = g[i];
    let mut r = (uz[(i, 0)] as i64) % gi;
    if r < 0 {
      r += gi;
    }
    h += prod * r;
  }
  h
}

// pub fn build_h_i<'a>(
//   u: &DMatrix<f64>,
//   g: &DMatrix<f64>,
//   digits: &'a Vec<DVector<f64>>,
// ) -> Result<HashMap<i32, &'a DVector<f64>>, OpError> {
//   let mut h_map: HashMap<i32, &DVector<f64>> = HashMap::new();
//   for digit in digits.iter() {
//     let key = hash_point(u, g, digit)?;
//     h_map.insert(key, digit);
//   }
//   // store s and products (or just products) instead of recalculating
//   Ok(h_map)
// }

// TODO: Set back to private
// pub fn get_congruent<'a>(
//   u: &DMatrix<f64>,
//   g: &DMatrix<f64>,
//   x: &DVector<f64>,
//   h_map: &HashMap<i32, &'a DVector<f64>>,
// ) -> Result<&'a DVector<f64>, OpError> {
//   // digits should could be a matrix, or at least a vec + int

//   let h_x = hash_point(u, g, x)?;
//   let digit_option = h_map.get(&h_x);
//   match digit_option {
//     Some(digit) => Ok(*digit),
//     None => Err(OpError::NoCongruentDigit(x.clone(), h_x)),
//   }
// }

pub fn phi<'a>(
  m_inv: &DMatrix<f64>,
  point: &DVector<f64>,
  digits: &SystemDigitsEnum,
) -> Result<DVector<f64>, OpError> {
  let congruent_digit = match digits.get_congruent(point) {
  Some(digit) => digit,
  None => return Err(OpError::NoCongruentDigit(point.clone()))
  };
  let diff = point - congruent_digit;
  let mut res = m_inv * diff;
  res.apply(|comp| *comp = comp.round());
  Ok(res)
}

pub fn get_loop_floyd<'a>(
  m_inv: &DMatrix<f64>,
  point: &DVector<f64>,
  digits: &SystemDigitsEnum,
  // h_map: &HashMap<i32, &'a DVector<f64>>,
) -> Result<Vec<DVector<f64>>, OpError> {
  let mut slow = phi(m_inv, point, digits)?;
  let mut fast = phi(m_inv, &slow, digits)?;

  while slow != fast {
    slow = phi(m_inv, &slow, digits)?;
    fast = phi(m_inv, &phi(m_inv, &fast, digits)?, digits)?;
  }

  let loop_start = slow.clone();

  let mut loop_elements = vec![loop_start.clone()];
  let mut current = phi(m_inv, &loop_start, digits)?;

  while current != loop_start {
    loop_elements.push(current.clone());
    current = phi(m_inv, &current, digits)?;
  }

  Ok(loop_elements)
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

fn find_c_gamma_spectral(m_inv: &DMatrix<f64>) -> Result<(usize, f64), OpError> {
  let norm_threshold: f64 = 0.01;
  let mut c: usize = 1;
  let inv_norm = spectral_norm(&m_inv);
  if inv_norm >= 1.0 {
    return Err(OpError::InvalidNorm(Norms::L2));
  }

  let mut m_pow = m_inv.clone();

  while spectral_norm(&m_pow) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - spectral_norm(&m_pow));

  Ok((c, gamma))
}

fn find_c_gamma_norm(m_inv: &DMatrix<f64>, norm: &impl Norm<f64>, norm_type: Norms) -> Result<(usize, f64), OpError> {
  let norm_threshold: f64 = 0.01;
  let mut c: usize = 1;
  let inv_norm = m_inv.apply_norm(norm);
  if inv_norm >= 1.0 {
    return Err(OpError::InvalidNorm(norm_type));
  }

  let mut m_pow = m_inv.clone();
  while m_pow.apply_norm(norm) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - m_pow.apply_norm(norm));

  Ok((c, gamma))
}

pub fn find_c_gamma(m_inv: &DMatrix<f64>, norm: Norms) -> Result<(usize, f64), OpError> {
  match norm {
    Norms::L2 => find_c_gamma_spectral(m_inv),
    Norms::Infinite => find_c_gamma_norm(m_inv, &UniformNorm, Norms::Infinite),
    Norms::L1 => find_c_gamma_norm(m_inv, &LpNorm(1), Norms::L1),
  }
}

pub fn get_cover_box(
  m_inv: &DMatrix<f64>,
  c: usize,
  gamma: f64,
  digits: &SystemDigitsEnum,
) -> Result<(Vec<i32>, Vec<i32>), OpError> {
  let mut m_pow = m_inv.clone();
  let dim = m_inv.ncols();

  let mut sum_xi: DVector<f64> = DVector::from_element(dim, 0.0);
  let mut sum_eta: DVector<f64> = DVector::from_element(dim, 0.0);

  for j in 0..c {
    let mut iter = digits.get_digits_iter();
    let first = iter.next().ok_or(OpError::EmptyDigits)?;

    let mut xi_j = &m_pow * &first;
    let mut eta_j = xi_j.clone();

    for digit in iter {
      let prod = &m_pow * &digit;
      for m in 0..dim {
        if prod[m] > xi_j[m] {
          xi_j[m] = prod[m];
        }
        if prod[m] < eta_j[m] {
          eta_j[m] = prod[m];
        }
      }
    }

    sum_xi += xi_j;
    sum_eta += eta_j;

    if j < c - 1 {
      m_pow = &m_pow * m_inv;
    }
  }


  // println!("Sum_xi: {:?}", sum_xi);
  // println!("Sum_eta: {:?}", sum_eta);

  let mut l: Vec<i32> = vec![0; dim];
  let mut u: Vec<i32> = vec![0; dim];
  for m in 0..dim {
    l[m] = (-1.0 * (&gamma * &sum_eta[m])).ceil() as i32;
    u[m] = (-1.0 * (&gamma * &sum_xi[m])).floor() as i32;
  }

  Ok((u, l))
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn get_congruent_test() {
  //   let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
  //   let d: Vec<DVector<f64>> = vec![
  //     DVector::from_row_slice(&[0.0, 0.0]),
  //     DVector::from_row_slice(&[1.0, 0.0]),
  //     DVector::from_row_slice(&[0.0, 1.0]),
  //     DVector::from_row_slice(&[0.0, -1.0]),
  //     DVector::from_row_slice(&[-6.0, 5.0]),
  //   ];
  //   let data: PreComputed<f64> = match pre_compute(base) {
  //     Ok(pre_computed) => pre_computed,
  //     Err(err) => {
  //       panic!("Error while precomputing: {:?}", err);
  //     }
  //   };
  //   let start = DVector::from_row_slice(&[0.0, 0.0]);
  //   let cong_value = match get_congruent(&data.u, &data.g, &start, &h_map) {
  //     Ok(map) => map,
  //     Err(err) => panic!("Failed to get cong value: {:?}", err),
  //   };
  //   assert_eq!(*cong_value, start);
  // }

  #[test]
  fn phi_test() -> Result<(), OpError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    let data: PreComputed<f64> = match pre_compute(base.clone()) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };

    let starts = vec![
      DVector::from_row_slice(&[-6.0, 5.0]),
      DVector::from_row_slice(&[-6.0, 4.0]),
      DVector::from_row_slice(&[-6.0, 3.0]),
    ];
    let expected = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[-2.0, 3.0]),
      DVector::from_row_slice(&[-2.0, 2.0]),
    ];
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d));
    for i in 0..3 {
      let res = phi(&data.m_inv, &starts[i], &digits)?;
      assert_eq!(res, expected[i]);
      println!("{} asserted.", i);
    }

    Ok(())
  }

  #[test]
  fn floyd_test() -> Result<(), OpError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    let data: PreComputed<f64> = match pre_compute(base.clone()) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d));

    let start: DVector<f64> = DVector::from_column_slice(&[-6.0, 3.0]);
    let expected = vec![
      DVector::from_column_slice(&[0.0, 0.0])
      ];
    let res = get_loop_floyd(&data.m_inv, &start, &digits)?;
    assert_eq!(expected, res);

    Ok(())
  }

  #[test]
  fn cover_box_test() -> Result<(), OpError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    let data: PreComputed<f64> = match pre_compute(base.clone()) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };
    let (c, gamma) = find_c_gamma(&data.m_inv, Norms::Infinite)?;
    println!("c: {:?}", c);
    let digits_enum = SystemDigitsEnum::Explicit(get_explicit(&base, d.clone()));
    let expected_box: (Vec<i32>, Vec<i32>) = (vec![-2, -6], vec![2, 1]);
    let cover_box = get_cover_box(&data.m_inv, c, gamma, &digits_enum)?;
    println!("Box: {:?}", cover_box);
    assert_eq!(expected_box, cover_box);

    Ok(())
  }
}
