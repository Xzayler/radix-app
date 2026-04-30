use std::sync::{Arc, Mutex};

use crate::{
  error::WorkerError,
  executor::algorithm::{
    math::satisfies_unit_condition,
    systems::{System, SystemEnum},
  },
};
use nalgebra::DVector;

use rayon::prelude::*;

fn loop_contains_point(loop_points: &[DVector<f64>], point: &DVector<f64>) -> bool {
  loop_points.contains(point)
}

fn get_all_loops(
  l_corner: &Vec<i32>,
  h_corner: &Vec<i32>,
  system: &SystemEnum,
) -> Result<Vec<Vec<DVector<f64>>>, WorkerError> {
  assert_eq!(l_corner.len(), h_corner.len());

  let dims = l_corner.len();

  let sizes: Vec<usize> = (0..dims)
    .map(|i| (h_corner[i] - l_corner[i] + 1) as usize)
    .collect();

  let total: usize = sizes.iter().product();
  let all_loops: Arc<Mutex<Vec<Vec<DVector<f64>>>>> = Arc::new(Mutex::new(Vec::new()));
  let errors: Arc<Mutex<Vec<WorkerError>>> = Arc::new(Mutex::new(Vec::new()));

  (0..total).into_par_iter().for_each(|mut idx| {
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
        let mut collected_errors =
          errors.lock().expect("errors mutex should not be poisoned");
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

  Ok(Arc::try_unwrap(all_loops)
    .expect("all_loops should have a single owner after rayon completes")
    .into_inner()
    .expect("all_loops mutex should not be poisoned"))
}

pub fn classification(system: &SystemEnum) -> Result<Vec<Vec<DVector<f64>>>, WorkerError> {
  let (l_corner, h_corner) = system.get_cover_box()?;
  println!("Box: {:?}, {:?}", l_corner, h_corner);
  let unique_loops: Vec<Vec<DVector<f64>>> = get_all_loops(&l_corner, &h_corner, system)?;
  Ok(unique_loops)
}

fn has_any_loop<'a>(l_corner: &Vec<i32>, h_corner: &Vec<i32>, system: &SystemEnum) -> bool {
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
          println!("Loop: {:?}", v);
          return true;
        }
        let loop_point = match v.get(0) {
          Some(point) => point,
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
      Err(err) => {
        println!("Error: {:?}", err);
        return true;
      }
    }
  })
}

pub fn decision(system: &SystemEnum) -> Result<bool, WorkerError> {
  if !satisfies_unit_condition(system.get_base()) {
    return Ok(false);
  }
  let (l_corner, h_corner) = system.get_cover_box()?;
  println!("Cover box {:?}, {:?}", l_corner, h_corner);
  let res = has_any_loop(&l_corner, &h_corner, system);

  Ok(!res)
}

pub fn get_loop_floyd<'a>(
  system: &SystemEnum,
  point: &DVector<f64>,
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

pub fn walk(
  system: &SystemEnum,
  start_point: DVector<f64>,
) -> Result<Vec<DVector<f64>>, WorkerError> {
  let mut walked_points = Vec::new();
  let mut point = start_point;

  loop {
    if loop_contains_point(&walked_points, &point) {
      walked_points.push(point);
      break;
    }

    walked_points.push(point.clone());
    point = system.phi(&point)?;
  }

  Ok(walked_points)
}

#[cfg(test)]
mod tests {
  use crate::executor::algorithm::{
    digits::{SystemDigitsEnum, get_adjoint, get_canonical, get_explicit, get_symmetric},
    norms::NormEnum,
    systems::GenericSystem,
    systems_factories::{BuilderContext, GenericFactory, SystemFactory},
  };

  use super::*;
  use nalgebra::DMatrix;


  fn build_explicit_system(
    dim: usize,
    base_vals: &[f64],
    digit_vecs: Vec<Vec<f64>>,
  ) -> SystemEnum {
    let base = DMatrix::from_row_slice(dim, dim, base_vals);
    let d: Vec<DVector<f64>> = digit_vecs
      .into_iter()
      .map(|v| DVector::from_row_slice(&v))
      .collect();
    let digits = SystemDigitsEnum::Explicit(get_explicit(&base, d).expect("digits"));
    SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite).expect("system"))
  }

  fn dvec(vals: &[f64]) -> DVector<f64> {
    DVector::from_row_slice(vals)
  }

  #[test]
  fn floyd_test() {
    let system = build_explicit_system(
      2,
      &[2.0, -1.0, 1.0, 2.0],
      vec![
        vec![0.0, 0.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![0.0, -1.0],
        vec![-6.0, 5.0],
      ]
    );
    let start: DVector<f64> = dvec(&[-6.0, 3.0]);
    let expected = vec![dvec(&[0.0, 0.0])];
    let res = get_loop_floyd(&system, &start).expect("loops");
    assert_eq!(expected, res);
  }

  #[test]
  fn classification_test() {
    let base = DMatrix::from_row_slice(2,2, &[3.0, 14.0, 7.0, 3.0]);
    let digits = get_canonical(&base).expect("canonical");
    let system = GenericSystem::new(
      base,
      SystemDigitsEnum::Canonical(digits),
      NormEnum::Infinite
    ).expect("system");

    let res = classification(&SystemEnum::Generic(system)).expect("loops");
    let expected_p_points = vec![
        dvec(&[2.0, -6.0]),
        dvec(&[0.0, -2.0]),
        dvec(&[1.0, -3.0]),
        dvec(&[0.0, -1.0]),
        dvec(&[2.0, -5.0]),
        dvec(&[1.0, -4.0]),
        dvec(&[-1.0, 0.0]),
        dvec(&[3.0, -7.0]),
        dvec(&[0.0, 0.0])
    ];

    let loop_lengths: Vec<usize> = res.clone().into_iter()
      .map(|l| l.len())
      .collect();
    assert_eq!(loop_lengths.len(), 3);
    assert!(loop_lengths.contains(&1));
    assert!(loop_lengths.contains(&2));
    assert!(loop_lengths.contains(&6));

    let p_points: Vec<DVector<f64>> = res.into_iter().flatten().collect();
    for point in p_points {
      assert!(expected_p_points.contains(&point));
    }
    
  }


  #[test]
  fn walk_test() {
    let system = build_explicit_system(
      2,
      &[2.0, -1.0, 1.0, 2.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, -1.0], vec![-6.0, 5.0]],
    );
    let res = walk(&system, dvec(&[-6.0, 3.0])).expect("path");
    let expected = vec![
      dvec(&[-6.0, 3.0]),
      dvec(&[-2.0, 2.0]),
      dvec(&[1.0, -2.0]),
      dvec(&[0.0, -1.0]),
      dvec(&[0.0, 0.0]),
      dvec(&[0.0, 0.0]),
    ];
    assert_eq!(res, expected);
  }

  #[test]
  fn decision_test() {
    let system = build_explicit_system(
      4,
      &[0.0, -2.0, 0.0, 0.0, 2.0, -2.0, 0.0, 0.0, 0.0, 0.0, 1.0, -2.0, 0.0, 0.0, 2.0, -1.0],
      vec![
        vec![0.0, 0.0, 0.0, 0.0],
        vec![1.0, 0.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0, 2.0],
        vec![1.0, 1.0, 1.0, 1.0],
        vec![-1.0, 0.0, -1.0, 0.0],
        vec![-2.0, 0.0, -2.0, 0.0],
        vec![-1.0, -1.0, -1.0, -1.0],
        vec![-2.0, -1.0, -2.0, -1.0],
        vec![2.0, -1.0, 2.0, -1.0],
        vec![-2.0, 1.0, -2.0, 1.0],
        vec![-1.0, -2.0, -1.0, -2.0],
        vec![-3.0, -3.0, -3.0, -3.0],
      ]
    );
    let res = match decision(&system) {
      Ok(b) => b,
      Err(_err) => panic!("error in decision"),
    };
    assert!(res);
  }

  #[test]
  fn decision_decimal_test() {
    let base: DMatrix<f64> = DMatrix::from_row_slice(1, 1, &[10.0]);

    let digits = SystemDigitsEnum::Symmetric(get_symmetric(&base).expect("msg"));
    let builder_ctx = BuilderContext {
      base: base,
      digits,
      norm: NormEnum::Infinite,
    };
    let system = GenericFactory.create(builder_ctx).expect("system");
    let res = match decision(&system) {
      Ok(b) => b,
      Err(_err) => panic!("error in decision"),
    };
    assert!(res);
  }

  #[test]
  fn walk_test_2() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[1.0, -2.0, 1.0, 1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![-1.0, 0.0]],
    );
    let res = walk(&system, dvec(&[3.0, 1.0]))?;
    let expected = vec![
      dvec(&[3.0, 1.0]),
      dvec(&[2.0, -1.0]),
      dvec(&[0.0, -1.0]),
      dvec(&[-1.0, 0.0]),
      dvec(&[0.0, 0.0]),
      dvec(&[0.0, 0.0]),
    ];
    assert_eq!(res, expected);
    Ok(())
  }

  #[test]
  fn walk_test_3() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      2,
      &[-1.0, -1.0, 1.0, -1.0],
      vec![vec![0.0, 0.0], vec![1.0, 0.0]],
    );
    let res = walk(&system, dvec(&[3.0, 1.0]))?;
    let expected = vec![
      dvec(&[3.0, 1.0]),
      dvec(&[-1.0, -2.0]),
      dvec(&[0.0, 2.0]),
      dvec(&[1.0, -1.0]),
      dvec(&[-1.0, 0.0]),
      dvec(&[1.0, 1.0]),
      dvec(&[0.0, -1.0]),
      dvec(&[0.0, 1.0]),
      dvec(&[1.0, 0.0]),
      dvec(&[0.0, 0.0]),
      dvec(&[0.0, 0.0]),
    ];
    assert_eq!(res, expected);
    Ok(())
  }

  #[test]
  fn walk_test_4() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[0.0, -2.0, 1.0, -2.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    let res = walk(&system, dvec(&[2.0, 1.0]))?;
    let expected = vec![
      dvec(&[2.0, 1.0]),
      dvec(&[-1.0, -1.0]),
      dvec(&[1.0, 1.0]),
      dvec(&[1.0, 0.0]),
      dvec(&[0.0, 0.0]),
      dvec(&[0.0, 0.0]),
    ];
    assert_eq!(res, expected);
    Ok(())
  }

  #[test]
  fn walk_test_5() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[0.0, 2.0, 1.0, 0.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    let res = walk(&system, dvec(&[-1.0, 0.0]))?;
    let expected = vec![
      dvec(&[-1.0, 0.0]),
      dvec(&[0.0, -1.0]),
      dvec(&[-1.0, 0.0]),
    ];
    assert_eq!(res, expected);
    Ok(())
  }

  #[test]
  fn walk_test_6() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      1,
      &[3.0],
      vec![vec![-2.0], vec![0.0], vec![2.0]],
    );
    let res = walk(&system, dvec(&[3.0]))?;
    assert_eq!(res, vec![dvec(&[3.0]), dvec(&[1.0]), dvec(&[1.0])]);

    let res2 = walk(&system, dvec(&[7.0]))?;
    assert_eq!(res2, vec![dvec(&[7.0]), dvec(&[3.0]), dvec(&[1.0]), dvec(&[1.0])]);
    Ok(())
  }

  #[test]
  fn walk_test_7() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(5, 5, &[
      0.0, 0.0, 0.0, 0.0, -7.0,
      1.0, 0.0, 0.0, 0.0,  6.0,
      0.0, 1.0, 0.0, 0.0,  0.0,
      0.0, 0.0, 1.0, 0.0,  0.0,
      0.0, 0.0, 0.0, 1.0,  0.0,
    ]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    let res = walk(&system, dvec(&[0.0, 1.0, 2.0, 3.0, 4.0]))?;
    let expected = vec![
      dvec(&[0.0, 1.0, 2.0, 3.0, 4.0]),
      dvec(&[1.0, 2.0, 3.0, 4.0, 0.0]),
      dvec(&[2.0, 3.0, 4.0, 0.0, 0.0]),
      dvec(&[3.0, 4.0, 0.0, 0.0, 0.0]),
      dvec(&[4.0, 0.0, 0.0, 0.0, 0.0]),
      dvec(&[0.0, 0.0, 0.0, 0.0, 0.0]),
      dvec(&[0.0, 0.0, 0.0, 0.0, 0.0]),
    ];
    assert_eq!(res, expected);
    Ok(())
  }

  #[test]
  fn decision_test_2() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[-3.0, 1.0, 1.0, -2.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_3() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[0.0, 2.0, 1.0, 0.0]);
    let digits = SystemDigitsEnum::Canonical(get_canonical(&base).expect("canonical"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert!(!decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_4() -> Result<(), WorkerError> {
    let system = build_explicit_system(1, &[3.0], vec![vec![0.0], vec![7.0], vec![2.0]]);
    assert!(!decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_5() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[2.0, -1.0, 1.0, 2.0]);
    let digits = SystemDigitsEnum::Adjoint(get_adjoint(&base).expect("adjoint"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_6() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(2, 2, &[3.0, -1.0, 1.0, 3.0]);
    let digits = SystemDigitsEnum::Adjoint(get_adjoint(&base).expect("adjoint"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::Infinite)?);
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_7() -> Result<(), WorkerError> {
    let system = build_explicit_system(4,
      &[
        0.0, -2.0, 0.0, 0.0,
        2.0, -2.0, 0.0, 0.0,
        0.0,  0.0, 1.0, -2.0,
        0.0,  0.0, 2.0, -1.0,
      ],
      vec![
        vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 0.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0, 2.0], vec![1.0, 1.0, 1.0, 1.0],
        vec![-1.0, 0.0, -1.0, 0.0], vec![-2.0, 0.0, -2.0, 0.0],
        vec![-1.0, -1.0, -1.0, -1.0], vec![-2.0, -1.0, -2.0, -1.0],
        vec![2.0, -1.0, 2.0, -1.0], vec![-2.0, 1.0, -2.0, 1.0],
        vec![-1.0, -2.0, -1.0, -2.0], vec![-3.0, -3.0, -3.0, -3.0],
      ]
    );
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_8() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      4,
      &[
        1.0, -2.0, 0.0, 0.0,
        2.0, -1.0, 0.0, 0.0,
        0.0,  0.0, 2.0, -2.0,
        0.0,  0.0, 2.0,  0.0,
      ],
      vec![
        vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 0.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0, 2.0], vec![1.0, 1.0, 1.0, 1.0],
        vec![-1.0, 0.0, -1.0, 0.0], vec![1.0, -1.0, 1.0, -1.0],
        vec![0.0, -1.0, 0.0, -1.0], vec![-2.0, 0.0, -2.0, 0.0],
        vec![-1.0, -1.0, -1.0, -1.0], vec![2.0, -1.0, 2.0, -1.0],
        vec![-1.0, -2.0, -1.0, -2.0], vec![0.0, -3.0, 0.0, -3.0],
      ]
    );
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_9() -> Result<(), WorkerError> {
    let system = build_explicit_system(
      4,
      &[
        -2.0,  1.0, 0.0, 0.0,
        -1.0, -1.0, 0.0, 0.0,
        0.0,  0.0, -2.0, 0.0,
        0.0,  0.0,  0.0, -2.0,
      ],
      vec![
        vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 0.0, 1.0, 0.0],
        vec![0.0, 2.0, 0.0, 2.0], vec![0.0, 1.0, 0.0, 1.0],
        vec![-2.0, 1.0, -2.0, 1.0], vec![1.0, -2.0, 1.0, -2.0],
        vec![-3.0, -1.0, -3.0, -1.0], vec![-2.0, 0.0, -2.0, 0.0],
        vec![-1.0, -1.0, -1.0, -1.0], vec![-2.0, -1.0, -2.0, -1.0],
        vec![-1.0, -2.0, -1.0, -2.0], vec![-3.0, -3.0, -3.0, -3.0],
      ]
    );
    assert!(decision(&system)?);
    Ok(())
  }

  #[test]
  fn decision_test_10() -> Result<(), WorkerError> {
    let base = DMatrix::from_row_slice(4, 4, &[
      0.0, 0.0, 0.0, -15.0,
      1.0, 0.0, 0.0,  -1.0,
      0.0, 1.0, 0.0,  -2.0,
      0.0, 0.0, 1.0,  -3.0,
    ]);
    let digits = SystemDigitsEnum::Symmetric(get_symmetric(&base).expect("symmetric"));
    let system = SystemEnum::Generic(GenericSystem::new(base, digits, NormEnum::L1)?);
    match decision(&system) {
      Ok(_) => panic!(),
      Err(err) => {
        assert!(matches!(err, WorkerError::InvalidNorm { .. }));
      }
    };
    Ok(())
  }
}
