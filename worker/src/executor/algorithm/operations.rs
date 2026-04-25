use std::sync::{Arc, Mutex};

use nalgebra::DVector;
use std::time::SystemTime;

use crate::{executor::algorithm::{lib::satisfies_unit_condition, systems::{System, SystemEnum}}, error::WorkerError};

use rayon::prelude::*;

fn loop_contains_point(loop_points: &[DVector<f64>], point: &DVector<f64>) -> bool {
  loop_points.contains(point)
}

fn get_all_loops(
  l_corner: &Vec<i32>,
  h_corner: &Vec<i32>,
  system: &SystemEnum
) -> Result<Vec<Vec<DVector<f64>>>, WorkerError> {
  assert_eq!(l_corner.len(), h_corner.len());

  let dims = l_corner.len();

  let sizes: Vec<usize> = (0..dims)
    .map(|i| (h_corner[i] - l_corner[i] + 1) as usize)
    .collect();

  let total: usize = sizes.iter().product();
  let all_loops: Arc<Mutex<Vec<Vec<DVector<f64>>>>> = Arc::new(Mutex::new(Vec::new()));
  let errors: Arc<Mutex<Vec<WorkerError>>> = Arc::new(Mutex::new(Vec::new()));

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

      match get_loop_floyd(system, &grid_point) {
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
  system: &SystemEnum,
) -> Result<Vec<Vec<DVector<f64>>>, WorkerError> {
  let start = SystemTime::now();
  println!("Started at {:?}", start);
  let (l_corner, h_corner) = system.get_cover_box()?;
  println!("Box: {:?}, {:?}", l_corner, h_corner);
  let unique_loops: Vec<Vec<DVector<f64>>> = get_all_loops(&l_corner, &h_corner, system)?;
  
  println!(
    "Duration: {:?}",
    SystemTime::now()
      .duration_since(start)
      .expect("time should go forward")
  );

  Ok(unique_loops)
}

fn has_any_loop<'a>(
  l_corner: &Vec<i32>,
  h_corner: &Vec<i32>,
  system: &SystemEnum
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
    let loop_set = get_loop_floyd(system, &grid_point);
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
  system: &SystemEnum
) -> Result<bool, WorkerError> {

  if !satisfies_unit_condition(system.get_base()) {
    return Ok(false);
  }

  let start = SystemTime::now();
  println!("Started at {:?}", start);
  let (l_corner, h_corner) = system.get_cover_box()?;
  println!("Cover box {:?}, {:?}", l_corner, h_corner);
  let res = has_any_loop(&l_corner, &h_corner, system);

  println!(
    "Duration: {:?}",
    SystemTime::now()
      .duration_since(start)
      .expect("time should go forward")
  );

  Ok(!res)
}

pub fn get_loop_floyd<'a>(
  system: &SystemEnum,
  point: &DVector<f64>
) -> Result<Vec<DVector<f64>>, WorkerError> {
  let mut slow = system.phi(point)?;
  let mut fast = system.phi(&system.phi(point)?)?;

  while slow != fast {
    slow = system.phi(&slow)?;
    fast = system.phi(&system.phi(&fast)?)?;
  }

  let loop_start = slow.clone();

  let mut loop_elements = vec![loop_start.clone()];
  let mut current = system.phi(&loop_start)?;

  while current != loop_start {
    loop_elements.push(current.clone());
    current = system.phi(&current)?;
  }

  Ok(loop_elements)
}

fn walk_recursive(
  system: &SystemEnum,
  point: DVector<f64>,
  walked_points: &mut Vec<DVector<f64>>,
) -> Result<(), WorkerError> {
  if loop_contains_point(walked_points, &point) {
    return Ok(());
  }

  walked_points.push(point.clone());
  let next_point = system.phi(&point)?;
  walk_recursive(system, next_point, walked_points)
}

pub fn walk(system: &SystemEnum, start_point: DVector<f64>) -> Result<Vec<DVector<f64>>, WorkerError> {
  let mut walked_points = Vec::new();
  walk_recursive(system, start_point, &mut walked_points)?;
  Ok(walked_points)
}

#[cfg(test)]
mod tests {
  use crate::executor::algorithm::{digits::{SystemDigitsEnum, get_explicit}, norms::NormEnum, systems::GenericSystem, systems_factories::{BuilderContext, GenericFactory, SystemFactory}};

use super::*;
  use nalgebra::DMatrix;

  #[test]
  fn floyd_test() -> Result<(), WorkerError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect("Error creating digits"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);

    let start: DVector<f64> = DVector::from_column_slice(&[-6.0, 3.0]);
    let expected = vec![
      DVector::from_column_slice(&[0.0, 0.0])
      ];
    let res = get_loop_floyd(&system, &start)?;
    assert_eq!(expected, res);

    Ok(())
  }

  #[test]
  fn walk_test() -> Result<(), WorkerError> {
    let base: DMatrix<f64> = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let d: Vec<DVector<f64>> = vec![
      DVector::from_row_slice(&[0.0, 0.0]),
      DVector::from_row_slice(&[1.0, 0.0]),
      DVector::from_row_slice(&[0.0, 1.0]),
      DVector::from_row_slice(&[0.0, -1.0]),
      DVector::from_row_slice(&[-6.0, 5.0]),
    ];
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect("Error creating digits"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);

    let start: DVector<f64> = DVector::from_column_slice(&[-6.0, 3.0]);
    let expected = vec![
      DVector::from_column_slice(&[-6.0, 3.0]),
      DVector::from_column_slice(&[-2.0, 2.0]),
      DVector::from_column_slice(&[1.0, -2.0]),
      DVector::from_column_slice(&[0.0, -1.0]),
      DVector::from_column_slice(&[0.0, 0.0]),
    ];
    let res = walk(&system, start)?;
    assert_eq!(expected, res);

    Ok(())
  }

  #[test]
  fn decision_test() -> Result<(), WorkerError> {
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

    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, digits).expect("digits should be fine"));
    let builder_ctx = BuilderContext {
      base: base,
      digits,
      norm: NormEnum::Infinite
    };
    let system = GenericFactory.create(builder_ctx)?;
    let res = match decision(&system) {
      Ok(b) => b,
      Err(_err) => panic!("error in decision")
    };
    assert!(res);
    Ok(())
  }
}
