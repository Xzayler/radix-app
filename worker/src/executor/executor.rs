use std::{error::Error, fmt};
use nalgebra::{DMatrix, DVector};

use crate::{db::{db, model::{DigitType, Job, JobType, NormType, System}}, executor::algorithm::{digits::{adjoint_digits, canonical_digits_vec, dense_digits, j_canonical_digits_vec, j_symmetric_digits_vec, shifted_canonical_digits_vec, symmetric_digits_vec}, models::Norms, operations::{classification, decision}}};


#[derive(Debug)]
pub enum WorkerError {
  InvalidInput(String),
  Database(String),
  Operation(String),
  Unhandled(String)
}

impl fmt::Display for WorkerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidInput(message) => write!(f, "Invalid input: {}", message),
      Self::Database(message) => write!(f, "Database error: {}", message),
      Self::Operation(message) => write!(f, "Operation error: {}", message),
      Self::Unhandled(message) => write!(f, "Unexpected error: {}", message),
    }
  }
}

impl Error for WorkerError {}

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
  let (base, digits) = build_system_params(&job.system, &norm)?;
  let output = build_job_output(&job, base, &digits, norm)?;
  
  println!("Updating with {:?}", output);
  match db::update_db_with_results(&pool, job_id, job.system.id, output.is_gns, output.signature).await {
    Ok(_) => (), 
    Err(err) => {
      return Err(WorkerError::Database(err.to_string()));
    }
  }

  Ok(())
}

fn build_system_params(
  system: &System,
  norm: &Norms
) -> Result<(DMatrix<f64>, Vec<DVector<f64>>), WorkerError> {
  let dim = system.dimension as usize;
  let float_base_values: Vec<f64> = system.base.iter().map(|el| *el as f64).collect();
  let base = DMatrix::from_row_slice(dim, dim, &float_base_values[..]);
  let det = base.determinant() as i64;

  let res: Vec<DVector<f64>> = match system.digit_type {
    DigitType::Explicit => match &system.digits {
      Some(digits) => digits.iter().map(|digit| build_na_vector(digit)).collect(),
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digits for explicit system.".into()))
      }
    },
    DigitType::Canonical => match canonical_digits_vec(dim, det.unsigned_abs()) {
      Ok(digits) => digits,
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    },
    DigitType::JCanonical => match system.digit_param {
      Some(param) => {
        let abs_determinant = det.unsigned_abs();

        match j_canonical_digits_vec(dim, abs_determinant, param as usize) {
          Ok(digits) => digits,
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for JCanonical system.".into()))
      }
    },
    DigitType::Symmetric => match symmetric_digits_vec(dim, det.unsigned_abs()) {
      Ok(digits) => digits,
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    },
    DigitType::JSymmetric => match system.digit_param {
      Some(param) => {
        let abs_determinant = det.unsigned_abs();

        match j_symmetric_digits_vec(dim, abs_determinant, param as usize) {
          Ok(digits) => digits,
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for JSymmetric system.".into()))
      }
    },
    DigitType::Shifted => match system.digit_param {
      Some(param) => {
        let abs_determinant = det.unsigned_abs();

        match shifted_canonical_digits_vec(dim, abs_determinant, 0, param as u32) {
          Ok(digits) => digits,
          Err(err) => {
            return Err(WorkerError::Unhandled(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for Shifted system.".into()))
      }
    },
    DigitType::Adjoined => match adjoint_digits(dim, det, &base) {
      Ok(digits) => digits,
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    },
    DigitType::Dense => match dense_digits(dim, det, &base, norm) {
      Ok(digits) => digits,
      Err(err) => {
        return Err(WorkerError::Unhandled(err.to_string()));
      }
    }
  };

  Ok((base, res))
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

fn build_job_output(job: &Job, base: DMatrix<f64>, digits: &Vec<DVector<f64>>, norm: Norms) -> Result<JobOutput, WorkerError> {

  let mut job_output: JobOutput = JobOutput { 
    is_gns: None,
    signature: None
  };

  match job.job_type {
    JobType::Classification => {
      println!("starting classification operation");
      let all_loops = match classification(base, digits, norm) {
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
      let res = match decision(base, digits, norm) {
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