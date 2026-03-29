use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nalgebra::{DMatrix, DVector};
use std::time::SystemTime;

use crate::executor::algorithm::digits::SystemDigitsEnum;
use crate::executor::algorithm::{
  functions::{
    find_c_gamma, get_cover_box, get_loop_floyd, pre_compute,
    PreComputed,
  },
  models::{Norms, OpError},
};

use rayon::prelude::*;

fn loop_contains_point(loop_points: &[DVector<f64>], point: &DVector<f64>) -> bool {
  loop_points.iter().any(|loop_point| loop_point == point)
}

fn get_all_loops(
  l_corner: &Vec<i32>,
  h_corner: &Vec<i32>,
  data: &PreComputed<f64>,
  digits: &SystemDigitsEnum
  // h_map: &HashMap<i32, DVector<f64>>,
) -> Result<Vec<Vec<DVector<f64>>>, OpError> {
  assert_eq!(l_corner.len(), h_corner.len());

  let dims = l_corner.len();

  let sizes: Vec<usize> = (0..dims)
    .map(|i| (h_corner[i] - l_corner[i] + 1) as usize)
    .collect();

  let total: usize = sizes.iter().product();
  let all_loops: Arc<Mutex<Vec<Vec<DVector<f64>>>>> = Arc::new(Mutex::new(Vec::new()));
  let errors: Arc<Mutex<Vec<OpError>>> = Arc::new(Mutex::new(Vec::new()));

  (0..total)
    .into_par_iter()
    .for_each(|mut idx| {
      let mut point = vec![0f64; dims];

      for d in (0..dims).rev() {
        let size = sizes[d];
        point[d] = (l_corner[d] as f64) + (idx % size) as f64;
        idx /= size;
      }

      let grid_point = DVector::from_column_slice(&point);
      match get_loop_floyd(&data.m_inv, &grid_point, digits) {
        Ok(loop_points) => {
          let Some(loop_point) = loop_points.first() else {
            return;
          };

          let mut stored_loops = all_loops
            .lock()
            .expect("all_loops mutex should not be poisoned");

          let already_discovered = stored_loops
            .iter()
            .any(|stored_loop| loop_contains_point(stored_loop, loop_point));

          if !already_discovered {
            stored_loops.push(loop_points);
          }
        }
        Err(err) => {
          let mut collected_errors = errors.lock().expect("errors mutex should not be poisoned");
          collected_errors.push(err);
        }
      }
    });

  if let Some(err) = errors
    .lock()
    .expect("errors mutex should not be poisoned")
    .pop()
  {
    return Err(err);
  }

  Ok(
    Arc::try_unwrap(all_loops)
      .expect("all_loops should have a single owner after rayon completes")
      .into_inner()
      .expect("all_loops mutex should not be poisoned"),
  )
}

pub fn classification(
  base: DMatrix<f64>,
  digits: &SystemDigitsEnum,
  norm: Norms,
) -> Result<Vec<Vec<DVector<f64>>>, OpError> {
  let data = pre_compute(base)?;
  let (c, gamma) = find_c_gamma(&data.m_inv, norm)?;
  let (l_corner, h_corner) = get_cover_box(&data.m_inv, c, gamma, digits)?;
  let unique_loops: Vec<Vec<DVector<f64>>> = get_all_loops(&l_corner, &h_corner, &data, &digits)?;

  Ok(unique_loops)
}

fn has_any_loop<'a>(
  l_corner: &Vec<i32>,
  h_corner: &Vec<i32>,
  data: &PreComputed<f64>,
  digits: &SystemDigitsEnum
) -> bool {
  assert_eq!(l_corner.len(), h_corner.len());
  let dims = l_corner.len();

  let sizes: Vec<usize> = (0..dims)
    .map(|i| (h_corner[i] - l_corner[i] + 1) as usize)
    .collect();

  let total: usize = sizes.iter().product();
  (0..total).into_par_iter().any(|mut idx| {
    let mut point = vec![0f64; dims];

    for d in (0..dims).rev() {
      let size = sizes[d];
      point[d] = (l_corner[d] as f64) + (idx % size) as f64;
      idx /= size;
    }

    let grid_point = DVector::from_column_slice(&point);
    // TODO: Handle errors
    let loop_set = get_loop_floyd(&data.m_inv, &grid_point, &digits);
    let zero_point: DVector<f64> = DVector::from_element(dims, 0.0);
    match loop_set {
      Ok(ref v) => {
        if v.len() != 1 {
          // Loop [0] has length 1, could be 0 point
          println!("Loop: {:?}", v);
          return true;
        }
        let loop_point = match v.get(0) {
          Some(point) => point,
          // TODO: Handle errors
          None => {
            println!("Error for: {:?}", v);
            return true;
          }
        };
        if *loop_point == zero_point {
          return false;
        }
        println!("Loop: {:?}", loop_point);
        return true;
      }
      // TODO: Handle errors
      Err(err) => {
        println!("Error: {:?}", err);
        return true;
      }
    }
  })
}

pub fn decision(
  base: DMatrix<f64>,
  digits: &SystemDigitsEnum,
  norm: Norms,
) -> Result<bool, OpError> {
  // TODO: Check if matrix is expansive, if not, false
  let start = SystemTime::now();
  println!("Started at {:?}", start);
  let data = pre_compute(base)?;
  let (c, gamma) = find_c_gamma(&data.m_inv, norm)?;
  let (l_corner, h_corner) = get_cover_box(&data.m_inv, c, gamma, digits)?;

  // TODO: Check if remainder value already exists, or size of h_map to be len of digits
  let res = has_any_loop(&l_corner, &h_corner, &data, &digits);

  println!(
    "Duration: {:?}",
    SystemTime::now()
      .duration_since(start)
      .expect("time should go forward")
  );

  Ok(!res)
}

#[cfg(test)]
mod tests {
  use crate::executor::algorithm::digits::get_explicit;

use super::*;

  #[test]
  fn decisionTest() -> Result<(), OpError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(
      4,
      4,
      &[
        0.0, -2.0, 0.0, 0.0, 2.0, -2.0, 0.0, 0.0, 0.0, 0.0, 1.0, -2.0, 0.0, 0.0, 2.0, -1.0,
      ],
    );
    let digits: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0, 0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0, 1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 2.0, 0.0, 2.0]),
      DVector::from_row_slice(&[1.0, 1.0, 1.0, 1.0]),
      DVector::from_row_slice(&[-1.0, 0.0, -1.0, 0.0]),
      DVector::from_row_slice(&[-2.0, 0.0, -2.0, 0.0]),
      DVector::from_row_slice(&[-1.0, -1.0, -1.0, -1.0]),
      DVector::from_row_slice(&[-2.0, -1.0, -2.0, -1.0]),
      DVector::from_row_slice(&[2.0, -1.0, 2.0, -1.0]),
      DVector::from_row_slice(&[-2.0, 1.0, -2.0, 1.0]),
      DVector::from_row_slice(&[-1.0, -2.0, -1.0, -2.0]),
      DVector::from_row_slice(&[-3.0, -3.0, -3.0, -3.0]),
    ];

    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, digits));
    let res = decision(base, &digits, Norms::Infinite)?;
    assert!(res);
    Ok(())
  }
}
