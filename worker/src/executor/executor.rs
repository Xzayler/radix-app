use nalgebra::{DMatrix, DVector};

use crate::{db::{db, model::{DbSystem, DigitType, Job, JobType, NormType}}, executor::algorithm::{digits::{SystemDigitsEnum, get_adjoint, get_canonical, get_dense, get_explicit, get_j_canonical, get_j_symmetric, get_shifted_canonical, get_symmetric}, models::{Norms, WorkerError}, operations::{classification, decision}, systems::SystemEnum, systems_factories::{BuilderContext, MatcherContext, SystemFactory, system_factories}}};

#[derive(Debug)]
struct JobOutput {
  is_gns: Option<bool>,
  signature: Option<Vec<i32>>
}

pub async fn run(job_id: i32) -> Result<(), WorkerError> {
  println!("Starting processing job id: {}", job_id);
  let pool = match db::connect().await {
    Ok(pool) => pool,
    Err(err) => {
      return Err(WorkerError::Database(err.to_string()));
    }
  };
  let job = match db::get_job(&pool, job_id).await {
    Ok(job) => job,
    Err(err) => {
      return Err(WorkerError::Database(err.to_string()));
    }
  };

  let norm = to_my_norm(&job.norm);
  let system = build_system(&job.system, &norm)?;
  let output = build_job_output(&job, &system)?;
  
  println!("Updating with {:?}", output);
  match db::update_db_with_results(&pool, job_id, job.system.id, output.is_gns, output.signature).await {
    Ok(_) => (), 
    Err(err) => {
      return Err(WorkerError::Database(err.to_string()));
    }
  }

  Ok(())
}

fn build_system(
  db_system: &DbSystem,
  norm: &Norms
// ) -> Result<(DMatrix<f64>, SystemDigitsEnum), WorkerError> {
) -> Result<SystemEnum, WorkerError> {
  let dim = db_system.dimension as usize;
  let float_base_values: Vec<f64> = db_system.base.iter().map(|el| *el as f64).collect();
  let base = DMatrix::from_row_slice(dim, dim, &float_base_values[..]);

  let digits: SystemDigitsEnum = match db_system.digit_type {
    DigitType::Explicit => match &db_system.digits {
      Some(digits) => {
        match get_explicit(&base, digits.iter().map(|digit| build_na_vector(digit)).collect()) {
          Ok(explicit_digits) => SystemDigitsEnum::Explicit(explicit_digits),
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digits for explicit system.".into()))
      }
    },
    DigitType::Canonical => match get_canonical(&base) {
      Ok(digits) => SystemDigitsEnum::Canonical(digits),
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    }
    DigitType::JCanonical => match db_system.digit_param {
      Some(param) => {
        match get_j_canonical(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Canonical(digits),
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for JCanonical system.".into()))
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
        match get_j_symmetric(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Symmetric(digits),
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for JSymmetric system.".into()))
      }
    },
    DigitType::Shifted => match db_system.digit_param {
      Some(param) => {
        match get_shifted_canonical(&base, 0, param as u32) {
          Ok(digits) => SystemDigitsEnum::Shifted(digits),
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for Shifted system.".into()))
      }
    },
    DigitType::Adjoined => match get_adjoint(&base) {
      Ok(digits) => SystemDigitsEnum::Adjoint(digits),
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    },
    DigitType::Dense => match get_dense(&base, norm) {
      Ok(digits) => SystemDigitsEnum::Dense(digits),
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    }
  };

  let matcher_ctx = MatcherContext {
    base: &base,
  };
  let systems_factory = choose_system_factory(matcher_ctx)?;

  let builder_ctx = BuilderContext {
    base: &base,
    digits,
    norm: norm.clone()
  };
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

fn build_na_vector(vec: &Vec<i32>) -> DVector<f64> {
  let v: Vec<f64> = vec.iter().map(|e| *e as f64).collect();
  DVector::from_vec(v)
}

fn to_my_norm(db_norm: &NormType) -> Norms {
  match db_norm {
    NormType::Infinite => Norms::Infinite,
    NormType::L1 => Norms::L1,
    NormType::L2 => Norms::L2,
  }
}

fn build_job_output(job: &Job, system: &SystemEnum) -> Result<JobOutput, WorkerError> {

  let mut job_output: JobOutput = JobOutput { 
    is_gns: None,
    signature: None
  };

  match job.job_type {
    JobType::Classification => {
      println!("starting classification operation");
      let all_loops = match classification(system) {
        Ok(all_loops) => all_loops,
        Err(err) => {
          return Err(WorkerError::Operation(err.to_string()));
        }
      };
      // TODO: Store in minio
      let loop_sizes: Vec<usize> = all_loops.iter().map(|loop_| loop_.len()).collect();
      let mut signature = vec![0; *loop_sizes.iter().max().unwrap_or(&0)];
      for size in &loop_sizes {
        signature[size - 1] += 1;
      }
      job_output.is_gns = Some(signature.len() == 1 && signature[0] == 1);
      job_output.signature = Some(signature);
    },
    JobType::Decision => {
      println!("Starting decision operation");
      let res = match decision(system) {
        Ok(res) => res,
        Err(err) => {
          return Err(WorkerError::Operation(err.to_string()));
        }
      };
      job_output.is_gns = Some(res);
      if res {
        job_output.signature = Some(vec![1]);
      }
    },
    JobType::Walk => ()
  };

  Ok(job_output)
}