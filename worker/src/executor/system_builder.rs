use nalgebra::{DMatrix, DVector};

use crate::{
  db::model::{DbSystem, DigitType, NormType},
  error::WorkerError,
  executor::algorithm::{
    digits::{
      SystemDigitsEnum, get_adjoint, get_canonical, get_explicit, get_j_canonical,
      get_j_symmetric, get_shifted_canonical, get_symmetric,
    },
    norms::{Norm, NormEnum},
    systems::SystemEnum,
    systems_factories::{BuilderContext, MatcherContext, SystemFactory, system_factories},
  },
};

pub(super) fn build_system(
  db_system: &DbSystem,
  db_norm: &NormType,
) -> Result<SystemEnum, WorkerError> {
  if db_system.dimension <= 0 {
    return Err(WorkerError::InvalidInput(
      "Dimension must be positive".to_string(),
    ));
  }

  let dim = db_system.dimension as usize;
  let float_base_values: Vec<f64> = db_system.base.iter().map(|el| *el as f64).collect();
  let base = DMatrix::from_row_slice(dim, dim, &float_base_values[..]);
  let norm = to_algorithm_norm(db_norm);

  let base_norm = norm.get_matrix_norm(&base);
  if base_norm <= 1.0 {
    return Err(WorkerError::InvalidNorm {
      norm: norm.to_string(),
      message: "Base is not expansive".to_string(),
    });
  }

  let digits: SystemDigitsEnum = match db_system.digit_type {
    DigitType::Explicit => match &db_system.digits {
      Some(digits) => {
        match get_explicit(
          &base,
          digits.iter().map(|digit| build_na_vector(digit)).collect(),
        ) {
          Ok(explicit_digits) => SystemDigitsEnum::Explicit(explicit_digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
          }
        }
      }
      None => {
        return Err(WorkerError::InvalidInput(
          "Couldn't find digits for explicit system.".into(),
        ));
      }
    },
    DigitType::Canonical => match get_canonical(&base) {
      Ok(digits) => SystemDigitsEnum::Canonical(digits),
      Err(err) => {
        return Err(WorkerError::InvalidInput(err.to_string()));
      }
    },
    DigitType::JCanonical => match db_system.digit_param {
      Some(param) => {
        if param < 0 {
          return Err(WorkerError::InvalidInput(
            "Parameter for canonical digits must not be negative.".to_string(),
          ));
        };
        match get_j_canonical(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Canonical(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
          }
        }
      }
      None => {
        return Err(WorkerError::InvalidInput(
          "Couldn't find digit parameter for JCanonical system.".into(),
        ));
      }
    },
    DigitType::Symmetric => match get_symmetric(&base) {
      Ok(digits) => SystemDigitsEnum::Symmetric(digits),
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    },
    DigitType::JSymmetric => match db_system.digit_param {
      Some(param) => {
        if param < 0 {
          return Err(WorkerError::InvalidInput(
            "Parameter for Jsymmetric digits must not be negative.".to_string(),
          ));
        };
        match get_j_symmetric(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Symmetric(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
          }
        }
      }
      None => {
        return Err(WorkerError::InvalidInput(
          "Couldn't find digit parameter for JSymmetric system.".into(),
        ));
      }
    },
    DigitType::Shifted => match db_system.digit_param {
      Some(param) => {
        if param < 0 {
          return Err(WorkerError::InvalidInput(
            "Parameter for shifted digits must not be negative.".to_string(),
          ));
        };
        match get_shifted_canonical(&base, 0, param as u32) {
          Ok(digits) => SystemDigitsEnum::Shifted(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
          }
        }
      }
      None => {
        return Err(WorkerError::InvalidInput(
          "Couldn't find digit parameter for Shifted system.".into(),
        ));
      }
    },
    DigitType::Adjoint => match get_adjoint(&base) {
      Ok(digits) => SystemDigitsEnum::Adjoint(digits),
      Err(err) => {
        return Err(WorkerError::InvalidInput(err.to_string()));
      }
    },
  };

  let matcher_ctx = MatcherContext { base: &base };
  let systems_factory = choose_system_factory(matcher_ctx)?;

  let builder_ctx = BuilderContext { base, digits, norm };
  let system = systems_factory.create(builder_ctx)?;

  Ok(system)
}

fn choose_system_factory(ctx: MatcherContext) -> Result<Box<dyn SystemFactory>, WorkerError> {
  for factory in system_factories() {
    if factory.matches(&ctx) {
      return Ok(factory);
    }
  }

  Err(WorkerError::NoMatchingSystem)
}

fn build_na_vector(vec: &[i32]) -> DVector<f64> {
  let v: Vec<f64> = vec.iter().map(|e| *e as f64).collect();
  DVector::from_vec(v)
}

fn to_algorithm_norm(db_norm: &NormType) -> NormEnum {
  match db_norm {
    NormType::Infinite => NormEnum::Infinite,
    NormType::L1 => NormEnum::L1,
    NormType::L2 => NormEnum::L2,
  }
}
