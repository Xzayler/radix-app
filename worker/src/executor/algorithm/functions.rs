use crate::executor::algorithm::models::{Norms, OpError};
use std::{collections::HashMap, vec};

extern crate nalgebra as na;
use algebraeon::nzq::{Integer, Natural, Rational};
use na::{DMatrix, DVector};
use algebraeon::rings::matrix::Matrix as Alg_Matrix;
use nalgebra::{EuclideanNorm, LpNorm, Norm, UniformNorm, coordinates::X};


struct PreComputed<T: Clone> {
  // m: DMatrix<T>,
  m_inv: DMatrix<T>,
  g: DMatrix<T>, // maybe just Vec<T>?
  u: DMatrix<T>,
  d: Vec<DVector<T>>,
  // grid_points: Vec<Vec<T>>
  // _i: Vec<u16> //precalculated h values for all digits?
}

fn get_smith_values(m: &DMatrix<f32>) -> (DMatrix<f32>, DMatrix<f32>) {
  // Assuming m is a square matrix
  let dim = m.ncols();
  let cols: Vec<Vec<Integer>> = m.column_iter()
  .map(|col| col.into_iter()
    .map(|elem| Integer::from(elem.clone() as i32)).collect())
  .collect();
  let alg_mat: Alg_Matrix<Integer> = Alg_Matrix::from_cols::<>(cols);
  let (u, g, _v, _k) = alg_mat.smith_algorithm();
  let um: DMatrix<f32> = DMatrix::from_row_slice(dim, dim, &(u.entries_list().into_iter().map(|i| i.into()).collect::<Vec<f32>>()));
  let gm: DMatrix<f32> = DMatrix::from_row_slice(dim, dim, &(g.entries_list().into_iter().map(|i| i.into()).collect::<Vec<f32>>()));
  return (um, gm)
}

fn pre_compute(m: DMatrix<f32>, d: Vec<DVector<f32>>) -> Result<PreComputed<f32>, OpError> {
  let (u, g) = get_smith_values(&m);

  let m_inv = match m.try_inverse() {
    Some(inv) => inv,
    None => return Err(OpError::NonInvertible)
  };

  let p = PreComputed { 
    // m: m,
    m_inv: m_inv,
    g: g,
    u: u,
    d: d
  };

  Ok(p)
}

fn h(u: &DMatrix<f32>, g: &DMatrix<f32>, z: &DVector<f32>) -> Result<i32, OpError> {
  let uz = u * z;
  // println!("UZ: {:?}", uz);
  // println!("coord: {:?}", uz[(1,0)]);

  let n = g.ncols();
  let mut s = 0;  // TODO: precompute s?
  while s < n && g[(s, s)] == 1.0 {
    s += 1;
  }

  let mut h = 0.0;
  for i in s..n {
    let mut prod: f32 = 1.0;
    for j in s..i {
      prod *= g[(j, j)]; // TODO: Precompute these products
    }
    let gi = g[(i, i)];
    let mut r = uz[(i, 0)] % gi;
    if r < 0.0 {
      r += gi;
    }
    h += prod * r;
  };
  Ok(h.round() as i32)
}

fn build_h_i<'a>(u: &DMatrix<f32>, g: &DMatrix<f32>, digits: &'a Vec<DVector<f32>>) -> Result<HashMap<i32, &'a DVector<f32>>, OpError> {
  let mut h_map: HashMap<i32, &DVector<f32>> = HashMap::new();
  for digit in digits.iter() {
    let key = h(u, g, digit)?;
    h_map.insert(key, digit);
  }
  // store s and products (or just products) instead of recalculating
  Ok(h_map)
}

fn get_congruent<'a>(u: &DMatrix<f32>, g: &DMatrix<f32>, x: &DVector<f32>, h_map: &HashMap<i32, &'a DVector<f32>>) // should cache?
    -> Result<&'a DVector<f32>, OpError> { // digits should could be a matrix, or at least a vec + int
  
  let h_x = h(u, g, x)?;
  let digit_option = h_map.get(&h_x);
  match digit_option {
    Some(digit) => Ok(*digit),
    None => Err(OpError::NoCongruentDigit)
  }
}

// h_i: maps an h(x) result -> index
pub fn phi<'a>(m_inv: &DMatrix<f32>, u: &DMatrix<f32>, g: &DMatrix<f32>, x: &DVector<f32>, h_map: &HashMap<i32, &'a DVector<f32>>)
      -> Result<DVector<f32>, OpError> {

  let congruent_digit = get_congruent(u, g, x, &h_map)?;
  let diff = x - congruent_digit;
  let res = m_inv * diff;
  Ok(res)
}

fn get_loop_floyd<'a>(m_inv: &DMatrix<f32>, u: &DMatrix<f32>, g: &DMatrix<f32>, x: &DVector<f32>, h_map: &HashMap<i32, &'a DVector<f32>>) 
    -> Result<Vec<DVector<f32>>, OpError> {

  let mut slow = phi(m_inv, u, g, x, h_map)?;
  let mut fast = phi(m_inv, u, g, &slow, h_map)?;

  while slow != fast {
    slow = phi(m_inv, u, g, &slow, h_map)?;
    fast = phi(m_inv, u, g, &phi(m_inv, u, g, &fast, h_map)?, h_map)?;
  }

  let mut ptr1 = x.clone();
  let mut ptr2 = slow;

  while ptr1 != ptr2 {
    ptr1 = phi(m_inv, u, g, &ptr1, h_map)?;
    ptr2 = phi(m_inv, u, g, &ptr2, h_map)?;
  }

  let loop_start = ptr1.clone();

  let mut loop_elements = vec![loop_start.clone()];
  let mut current = phi(m_inv, u, g, &loop_start, h_map)?;

  while current != loop_start {
      loop_elements.push(current.clone());
      current = phi(m_inv, u, g, &current, h_map)?;
  }

  Ok(loop_elements)
}

fn spectral_norm(m: &DMatrix<f32>) -> f32 {
  let svd = m.clone().svd(false, false);
  svd.singular_values[0]
}

fn find_c_gamma_spectral(m_inv: &DMatrix<f32>) -> Result<(usize, f32), OpError> {
  let norm_threshold: f32 = 0.01;
  let mut c: usize = 1;
  let inv_norm = spectral_norm(&m_inv);
  if inv_norm >= 1.0 {
    return Err(OpError::InvalidNorm(Norms::Uniform));
  }

  let mut m_pow = m_inv.clone();

  while spectral_norm(&m_pow) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - spectral_norm(&m_pow));

  Ok((c, gamma))
}

fn find_c_gamma_norm(m_inv: &DMatrix<f32>, norm: &impl Norm<f32>) -> Result<(usize, f32), OpError> {
  let norm_threshold: f32 = 0.01;
  let mut c: usize = 1;
  let inv_norm = m_inv.apply_norm(norm);
  if inv_norm >= 1.0 {
    return Err(OpError::InvalidNorm(Norms::Uniform));
  }

  let mut m_pow = m_inv.clone();
  while m_pow.apply_norm(norm) >= norm_threshold {
    c += 1;
    m_pow = &m_pow * m_inv;
  }

  let gamma = 1.0 / (1.0 - m_pow.apply_norm(norm));

  Ok((c, gamma))
}

fn find_c_gamma(m_inv: &DMatrix<f32>, norm: Norms) -> Result<(usize, f32), OpError> {
  match norm {
    Norms::Spectral => {
      find_c_gamma_spectral(m_inv)
    },
    Norms::Uniform => {
      find_c_gamma_norm(m_inv, &UniformNorm)
    },
    Norms::L1 => {
      find_c_gamma_norm(m_inv, &LpNorm(1))
    }
  }
}

fn get_cover_box(m_inv: &DMatrix<f32>, c: usize, gamma: f32, digits: &Vec<DVector<f32>>)
    -> Result<(Vec<i32>, Vec<i32>), OpError> {
  let mut m_pow = m_inv.clone();
  let dim = m_inv.ncols();

  let mut xi: Vec<DVector<f32>> = vec![DVector::from_element(0, 1.0); c];
  let mut eta: Vec<DVector<f32>> = vec![DVector::from_element(0, 1.0); c];
  for j in 0..c {
    let first_prod = &m_pow * &digits[0];
    xi[j] = first_prod.clone();
    eta[j] = first_prod.clone();

    for d_ind in 1..digits.len() {
      let prod = &m_pow * &digits[d_ind];
      for m in 0..dim {
        if prod[m] > xi[j][m] {
          xi[j][m] = prod[m].clone();
        }
        if prod[m] < eta[j][m] {
          eta[j][m] = prod[m].clone();
        }
      }
    }
    if j < (c-1) {
      m_pow = &m_pow * m_inv; 
    }
  }
  println!("Xi: {:?}", xi);
  println!("Eta: {:?}", eta);

  let mut sum_xi: DVector<f32> = DVector::from_element(dim, 0.0);
  let mut sum_eta: DVector<f32> = DVector::from_element(dim, 0.0);
  println!("Sum_xi: {:?}", sum_xi);
  println!("Sum_eta: {:?}", sum_xi);
  for (_j, vec) in xi.into_iter().enumerate() {
    sum_xi += vec;
  }
  for (_j, vec) in eta.into_iter().enumerate() {
    sum_eta += vec;
  }

  println!("Sum_xi: {:?}", sum_xi);
  println!("Sum_eta: {:?}", sum_eta);

  // let gamma = 1.0 / (1.0 - spectral_norm());
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

  #[test]
  fn get_congruent_test() {
    let base: DMatrix<f32> = DMatrix::from_row_slice(2,2, &[
      2.0, -1.0,
      1.0, 2.0
      ]);
    let d: Vec<DVector<f32>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
      ];
    let data: PreComputed<f32> = match pre_compute(base, d) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };

    let h_map = match build_h_i(&data.u, &data.g, &data.d) {
      Ok(map) =>  map,
      Err(err) => {
        panic!("Error while building map: {:?}", err);
      }
    };
    let start = DVector::from_row_slice(&[0.0, 0.0]);
    let cong_value = match get_congruent(&data.u, &data.g, &start, &h_map) {
      Ok(map) => map,
      Err(err) => panic!("Failed to get cong value: {:?}", err)
    };
    assert_eq!(*cong_value, start);
  }

  #[test]
  fn phi_test() -> Result<(), OpError> {
    let base: DMatrix<f32> = DMatrix::from_row_slice(2,2, &[
      2.0, -1.0,
      1.0, 2.0
      ]);
    let d: Vec<DVector<f32>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
      ];
    let data: PreComputed<f32> = match pre_compute(base, d) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };

    let h_map = match build_h_i(&data.u, &data.g, &data.d) {
      Ok(map) => map,
      Err(err) => {
        panic!("Error while building map: {:?}", err);
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
    for i in 0..3 {
      let res = phi(&data.m_inv, &data.u, &data.g, &starts[i], &h_map)?;
      assert_eq!(res, expected[i]);
      println!("{} asserted.", i);
    }

    Ok(())  
  }

  #[test]
  fn floyd_test() -> Result<(), OpError> {
    let base: DMatrix<f32> = DMatrix::from_row_slice(2,2, &[
      2.0, -1.0,
      1.0, 2.0
      ]);
    let d: Vec<DVector<f32>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
      ];
    let data: PreComputed<f32> = match pre_compute(base, d) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };

    let h_map = match build_h_i(&data.u, &data.g, &data.d) {
      Ok(map) => map,
      Err(err) => {
        panic!("Error while building map: {:?}", err);
      }
    };

    let start: DVector<f32> = DVector::from_column_slice(&[-6.0, 3.0]);
    let expected = vec![
      DVector::from_column_slice(&[0.0, 0.0]),
      DVector::from_column_slice(&[0.0, 0.0])
      ];
    let res = get_loop_floyd(&data.m_inv, &data.u, &data.g, &start, &h_map)?;
    println!("Result: {:?}", res);
    
    Ok(())
  }

  #[test]
  fn cover_box_test() -> Result<(), OpError> {
    let base: DMatrix<f32> = DMatrix::from_row_slice(2,2, &[
      2.0, -1.0,
      1.0, 2.0
      ]);
    let d: Vec<DVector<f32>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
      ];
    let data: PreComputed<f32> = match pre_compute(base, d) {
      Ok(pre_computed) => pre_computed,
      Err(err) => {
        panic!("Error while precomputing: {:?}", err);
      }
    };
    let (c, gamma) = find_c_gamma(&data.m_inv, Norms::Spectral)?;
    println!("c: {:?}", c);
    let expected_box: (Vec<i32>, Vec<i32>) = (vec![-2, -6], vec![2, 1]);
    let cover_box = get_cover_box(&data.m_inv, c, gamma, &data.d)?;
    println!("Box: {:?}", cover_box);
    assert_eq!(expected_box, cover_box);

    Ok(())
  }
}