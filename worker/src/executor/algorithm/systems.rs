use std::collections::HashMap;

use nalgebra::{DMatrix, DVector};

use crate::executor::algorithm::{digits::{SystemDigits, SystemDigitsEnum}, lib::{build_h_i, find_c_gamma, get_smith_data, hash_point}, models::{Norms, WorkerError}};

pub trait System {
  fn get_digits(&self) -> &SystemDigitsEnum;
  fn get_cover_box(&self) -> Result<(Vec<i32>, Vec<i32>), WorkerError>;
  fn phi(&self, point: &DVector<f64>) -> Result<DVector<f64>, WorkerError>;
}

pub enum SystemEnum {
  Generic(GenericSystem)
}

impl System for SystemEnum {
  fn get_digits(&self) -> &SystemDigitsEnum {
    match self {
      SystemEnum::Generic(s) => s.get_digits()
    }
  }

  fn get_cover_box(&self) -> Result<(Vec<i32>, Vec<i32>), WorkerError> {
    match self {
      SystemEnum::Generic(s) => s.get_cover_box()
    }
  }

  fn phi(&self, point: &DVector<f64>) -> Result<DVector<f64>, WorkerError> {
    match self {
      SystemEnum::Generic(s) => s.phi(point)
    }
  }
}

pub struct GenericSystem {
  dim: usize,
  digits: SystemDigitsEnum,
  m_inv: DMatrix<f64>,
  norm: Norms,
  h_map: HashMap<i64, DVector<f64>>,
  u: DMatrix<f64>,
  g_vec: Vec<i64>,
  s: usize,
  g_prods: Vec<i64>
}

impl GenericSystem {
  pub fn new(base: &DMatrix<f64>, digits: SystemDigitsEnum, norm: Norms) -> Result<Self, WorkerError> {
    let dim = base.ncols();

    let m_inv = match base.clone().try_inverse() {
      Some(inv) => inv,
      None => return Err(WorkerError::NonInvertibleBase),
    };

    let (u, g_vec) = get_smith_data(&base);

    let g_prods: Vec<i64> = g_vec.iter()
      .scan(1i64, |acc, &x| {
          *acc *= x;
          Some(*acc)
      })
      .collect();

    let mut s = 0;
    while s < dim && g_vec[s] == 1 {
      s += 1;
    }
    let h_map = build_h_i(dim, s, &u, &g_vec, &g_prods, &digits);


    Ok(GenericSystem { dim, digits, m_inv, norm, h_map, u: u, g_vec: g_vec, s, g_prods})
  }

  pub fn valid_for() -> bool {
    true
  }
}


impl System for GenericSystem {
  fn get_digits(&self) -> &SystemDigitsEnum {
    &self.digits
  }

  fn get_cover_box(&self) -> Result<(Vec<i32>, Vec<i32>), WorkerError> {
    let (c, gamma) = find_c_gamma(&self.m_inv, &self.norm)?;
    let m_inv = &self.m_inv;
    let mut m_pow = m_inv.clone();
    let dim = m_inv.ncols();

    let mut sum_xi: DVector<f64> = DVector::from_element(dim, 0.0);
    let mut sum_eta: DVector<f64> = DVector::from_element(dim, 0.0);

    for j in 0..c {
      let mut iter = self.digits.get_digits_iter();
      let first = iter.next().ok_or(WorkerError::Unhandled("No digits found".to_string()))?;

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

    let mut l: Vec<i32> = vec![0; dim];
    let mut u: Vec<i32> = vec![0; dim];
    for m in 0..dim {
      l[m] = (-1.0 * (&gamma * &sum_eta[m])).ceil() as i32;
      u[m] = (-1.0 * (&gamma * &sum_xi[m])).floor() as i32;
    }

    Ok((u, l))
  }

  fn phi(&self, point: &DVector<f64>) -> Result<DVector<f64>, WorkerError> {
    let point_hash = hash_point(self.dim, self.s, &self.u, &self.g_vec, &self.g_prods, point);
    
    let congruent_digit = match self.h_map.get(&point_hash) {
      Some(digit) => digit,
      None => return Err(WorkerError::NoCongruentDigit(point.clone()))
    };
    let diff = point - congruent_digit;
    let mut res = &self.m_inv * diff;
    res.apply(|el| *el = el.round());
    Ok(res)
  }

}

mod tests {
  use super::*;
  use crate::executor::algorithm::{digits::get_explicit, models::WorkerError};

  #[test]
  fn generic_system_phi_test() -> Result<(), WorkerError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
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
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect(""));
    let system = SystemEnum::Generic(GenericSystem::new(&base, digits, Norms::Infinite)?);

    for i in 0..3 {
      let res = system.phi(&starts[i])?;
      assert_eq!(res, expected[i]);
      println!("{} asserted.", i);
    }

    Ok(())
  }

  #[test]
  fn cover_box_test() -> Result<(), WorkerError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    // let (c, gamma) = find_c_gamma(&data.m_inv, Norms::Infinite)?;
    // println!("c: {:?}", c);
    let digits_enum = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect(""));
    let system = SystemEnum::Generic(GenericSystem::new(&base, digits_enum, Norms::Infinite)?);
    let expected_box: (Vec<i32>, Vec<i32>) = (vec![-2, -6], vec![2, 1]);
    let cover_box = system.get_cover_box()?;
    println!("Box: {:?}", cover_box);
    assert_eq!(expected_box, cover_box);

    Ok(())
  }
}