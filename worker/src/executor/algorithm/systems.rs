use std::collections::HashMap;

use nalgebra::{DMatrix, DVector};

use crate::{
  error::WorkerError,
  executor::algorithm::{
    digits::{SystemDigits, SystemDigitsEnum},
    math::{build_h_i, find_c_gamma, get_smith_data, hash_point},
    norms::NormEnum,
  },
};

pub trait System {
  fn get_base(&self) -> &DMatrix<f64>;
  fn get_digits(&self) -> &SystemDigitsEnum;
  fn get_cover_box(&self) -> Result<(Vec<i32>, Vec<i32>), WorkerError>;
  fn phi(&self, point: &DVector<f64>) -> Result<DVector<f64>, WorkerError>;
}

pub enum SystemEnum {
  Generic(GenericSystem),
}

impl System for SystemEnum {
  fn get_base(&self) -> &DMatrix<f64> {
    match self {
      SystemEnum::Generic(s) => s.get_base(),
    }
  }

  fn get_digits(&self) -> &SystemDigitsEnum {
    match self {
      SystemEnum::Generic(s) => s.get_digits(),
    }
  }

  fn get_cover_box(&self) -> Result<(Vec<i32>, Vec<i32>), WorkerError> {
    match self {
      SystemEnum::Generic(s) => s.get_cover_box(),
    }
  }

  fn phi(&self, point: &DVector<f64>) -> Result<DVector<f64>, WorkerError> {
    match self {
      SystemEnum::Generic(s) => s.phi(point),
    }
  }
}

pub struct GenericSystem {
  dim: usize,
  base: DMatrix<f64>,
  digits: SystemDigitsEnum,
  m_inv: DMatrix<f64>,
  norm: NormEnum,
  h_map: HashMap<i64, DVector<f64>>,
  u: DMatrix<f64>,
  g_vec: Vec<i64>,
  s: usize,
  g_prods: Vec<i64>,
}

impl GenericSystem {
  pub fn new(
    base: DMatrix<f64>,
    digits: SystemDigitsEnum,
    norm: NormEnum,
  ) -> Result<Self, WorkerError> {
    let dim = base.ncols();

    let m_inv = match base.clone().try_inverse() {
      Some(inv) => inv,
      None => return Err(WorkerError::NonInvertibleBase),
    };

    let (u, g_vec) = get_smith_data(&base);

    let g_prods: Vec<i64> = g_vec
      .iter()
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

    let abs_det = (base.determinant() as i64).unsigned_abs() as usize;
    if h_map.len() != abs_det {
      return Err(WorkerError::InvalidInput(
        "Digits don't form full residue system".to_string(),
      ));
    }

    Ok(GenericSystem {
      dim,
      base,
      digits,
      m_inv,
      norm,
      h_map,
      u: u,
      g_vec: g_vec,
      s,
      g_prods,
    })
  }

  pub fn valid_for() -> bool {
    true
  }
}

impl System for GenericSystem {
  fn get_base(&self) -> &DMatrix<f64> {
    &self.base
  }

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
      let first = iter
        .next()
        .ok_or(WorkerError::Unhandled("No digits found".to_string()))?;

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
      None => return Err(WorkerError::NoCongruentDigit(point.clone())),
    };
    let diff = point - congruent_digit;
    let mut res = &self.m_inv * diff;
    res.apply(|el| *el = el.round());
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use crate::executor::algorithm::digits::{get_adjoint, get_canonical, get_explicit, get_symmetric};

  use super::*;

  fn build_explicit_system(
    dim: usize,
    base_vals: &[f64],
    digit_vecs: Vec<Vec<f64>>,
  ) -> Result<SystemEnum, WorkerError> {
    let base = DMatrix::from_row_slice(dim, dim, base_vals);
    let d: Vec<DVector<f64>> = digit_vecs
      .into_iter()
      .map(|v| DVector::from_row_slice(&v))
      .collect();
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect("digits"));
    Ok(SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?))
  }

  #[test]
  fn generic_system_phi_test_1() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[2.0, -1.0, 1.0, 2.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, -1.0], vec![-6.0, 5.0]],
    )?;
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[-6.0, 5.0]))?,
      DVector::from_row_slice(&[0.0, 0.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[-6.0, 4.0]))?,
      DVector::from_row_slice(&[-2.0, 3.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[-6.0, 3.0]))?,
      DVector::from_row_slice(&[-2.0, 2.0])
    );
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_2() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      1,
      &[3.0],
      vec![vec![-2.0], vec![0.0], vec![2.0]],
    )?;
    assert_eq!(system.phi(&DVector::from_row_slice(&[1.0]))?, DVector::from_row_slice(&[1.0]));
    assert_eq!(system.phi(&DVector::from_row_slice(&[0.0]))?, DVector::from_row_slice(&[0.0]));
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_3() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[1.0, -2.0, 1.0, 1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![-1.0, 0.0]],
    )?;
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[3.0, 2.0]))?,
      DVector::from_row_slice(&[2.0, 0.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[1.0, 3.0]))?,
      DVector::from_row_slice(&[2.0, 1.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[4.0, 3.0]))?,
      DVector::from_row_slice(&[3.0, 0.0])
    );
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_4() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[-1.0, -1.0, 1.0, -1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0]],
    )?;
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[3.0, 2.0]))?,
      DVector::from_row_slice(&[0.0, -2.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[1.0, 3.0]))?,
      DVector::from_row_slice(&[1.0, -2.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[4.0, 3.0]))?,
      DVector::from_row_slice(&[0.0, -3.0])
    );
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_canonical() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[0.0, -2.0, 1.0, -2.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    let zero = DVector::from_row_slice(&[0.0, 0.0]);
    assert_eq!(system.phi(&DVector::from_row_slice(&[0.0, 0.0]))?, zero);
    assert_eq!(system.phi(&DVector::from_row_slice(&[1.0, 0.0]))?, zero);
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_cycle() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[0.0, 2.0, 1.0, 0.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[-1.0, -1.0]))?,
      DVector::from_row_slice(&[-1.0, -1.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[-1.0, 0.0]))?,
      DVector::from_row_slice(&[0.0, -1.0])
    );
    Ok(())
  }

   #[test]
  fn cover_box_1() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[2.0, -1.0, 1.0, 2.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, -1.0], vec![-6.0, 5.0]],
    )?;
    assert_eq!(system.get_cover_box()?, (vec![-2, -6], vec![2, 1]));
    Ok(())
  }

  #[test]
  fn cover_box_2() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      1,
      &[3.0],
      vec![vec![-2.0], vec![0.0], vec![2.0]],
    )?;
    assert_eq!(system.get_cover_box()?, (vec![-1], vec![1]));
    Ok(())
  }

  #[test]
  fn cover_box_3() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[1.0, -2.0, 1.0, 1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![-1.0, 0.0]],
    )?;
    assert_eq!(system.get_cover_box()?, (vec![-1, -1], vec![1, 1]));
    Ok(())
  }

  #[test]
  fn cover_box_4() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[-1.0, -1.0, 1.0, -1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0]],
    )?;
    assert_eq!(system.get_cover_box()?, (vec![-1, -1], vec![1, 1]));
    Ok(())
  }

  #[test]
  fn generic_system_phi_test_symmetric() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[1.0, -2.0, 1.0, 1.0]);
    let digits = SystemDigitsEnum::Symmetric(get_symmetric(&base).expect("symmetric"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[3.0, 2.0]))?,
      DVector::from_row_slice(&[2.0, 0.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[1.0, 3.0]))?,
      DVector::from_row_slice(&[2.0, 1.0])
    );
    assert_eq!(
      system.phi(&DVector::from_row_slice(&[4.0, 3.0]))?,
      DVector::from_row_slice(&[3.0, 0.0])
    );
    Ok(())
  }
}
