use nalgebra::{DMatrix, DVector};

use crate::{db::{db::{self, update_db_with_job_error}, model::{DbSystem, DigitType, Job, JobType, NormType}}, error::WorkerError, executor::algorithm::{norms::{Norm, NormEnum}, operations::walk}};
use crate::executor::algorithm::{digits::{SystemDigitsEnum, get_adjoint, get_canonical, get_explicit, get_j_canonical, get_j_symmetric, get_shifted_canonical, get_symmetric}, operations::{classification, decision}, systems::SystemEnum, systems_factories::{BuilderContext, MatcherContext, SystemFactory, system_factories}};
use crate::minio::minio::{upload_loop_results, upload_path_result};

#[derive(Debug)]
struct JobOutput {
  is_gns: Option<bool>,
  signature: Option<Vec<i32>>,
  all_loops: Option<Vec<Vec<DVector<f64>>>>,
  path: Option<Vec<DVector<f64>>>
}

pub async fn run(job_id: i32) -> () {
  let pool = match db::connect().await {
    Ok(pool) => pool,
    Err(err) => {
      panic!("Can't connect to database. {err}");
    }
  };
  let job = match db::get_job(&pool, job_id).await {
    Ok(job) => job,
    Err(err) => {
      panic!("Can't get job. {err}")
    }
  };

  let norm = to_my_norm(&job.norm);
  let system = match build_system(&job.system, norm) {
    Ok(system) => system,
    Err(err) => {
      let res = update_db_with_job_error(&pool, job_id, err.to_string()).await;
      if let Err(err) = res {
        panic!("Couldn't update job {job_id} with error {err}")
      }
      return ();
    }
  };
  let output = match build_job_output(&job, &system) {
    Ok(output) => output,
    Err(err) => {
      let res = update_db_with_job_error(&pool, job_id, err.to_string()).await;
      if let Err(err) = res {
        panic!("Couldn't update job {job_id} with error {err}")
      }
      return ();
    }
  };
  
  let mut output_uri = None;
  if let Some(loops) = output.all_loops {
    println!("Uploading to minio.");
    output_uri = match upload_loop_results(job_id, &loops).await {
      Ok(uri) => Some(uri),
      Err(err) => {
        panic!("Couldn't upload file. {err}")
      }
    }
  }

  if let Some(path) = output.path {
    println!("Uploading to minio");
    output_uri = match upload_path_result(job_id, &path).await {
      Ok(uri) => Some(uri),
      Err(err) => {
        panic!("Couldn't upload file. {err}")
      }
    }
  }

  match db::update_db_with_results(&pool, job_id, job.system.id, output.is_gns, output.signature, output_uri).await {
    Ok(_) => (), 
    Err(err) => {
      panic!("Can't update db with results. {err}");
    }
  }
}

fn build_system(
  db_system: &DbSystem,
  norm: NormEnum
) -> Result<SystemEnum, WorkerError> {
  if db_system.dimension <= 0 {
    return Err(WorkerError::InvalidInput("Dimension must be positive".to_string()));
  }
  let dim = db_system.dimension as usize;
  let float_base_values: Vec<f64> = db_system.base.iter().map(|el| *el as f64).collect();
  let base = DMatrix::from_row_slice(dim, dim, &float_base_values[..]);

  let base_norm = norm.get_matrix_norm(&base);
  if base_norm <= 1.0 {
    return Err(WorkerError::InvalidNorm {norm: norm.to_string(), message: "Base is not expansive".to_string()});
  }

  let digits: SystemDigitsEnum = match db_system.digit_type {
    DigitType::Explicit => match &db_system.digits {
      Some(digits) => {
        match get_explicit(&base, digits.iter().map(|digit| build_na_vector(digit)).collect()) {
          Ok(explicit_digits) => SystemDigitsEnum::Explicit(explicit_digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
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
        return Err(WorkerError::InvalidInput(err.to_string()));
      }
    }
    DigitType::JCanonical => match db_system.digit_param {
      Some(param) => {
        if param < 0 {
          return Err(WorkerError::InvalidInput("Parameter for canonical digits must not be negative.".to_string()))
        };
        match get_j_canonical(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Canonical(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
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
        if param < 0 {
          return Err(WorkerError::InvalidInput("Parameter for j-symmetric digits must not be negative.".to_string()))
        };
        match get_j_symmetric(&base, param as usize) {
          Ok(digits) => SystemDigitsEnum::Symmetric(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
          }
        }
      },
      None => {
        return Err(WorkerError::InvalidInput("Couldn't find digit parameter for JSymmetric system.".into()))
      }
    },
    DigitType::Shifted => match db_system.digit_param {
      Some(param) => {
        if param < 0 {
          return Err(WorkerError::InvalidInput("Parameter for shifted digits must not be negative.".to_string()))
        };
        match get_shifted_canonical(&base, 0, param as u32) {
          Ok(digits) => SystemDigitsEnum::Shifted(digits),
          Err(err) => {
            return Err(WorkerError::InvalidInput(err.to_string()));
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
        return Err(WorkerError::InvalidInput(err.to_string()));
      }
    }
  };

  let matcher_ctx = MatcherContext {
    base: &base,
  };
  let systems_factory = choose_system_factory(matcher_ctx)?;

  let builder_ctx = BuilderContext {
    base: base,
    digits,
    norm: norm
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

fn to_my_norm(db_norm: &NormType) -> NormEnum {
  match db_norm {
    NormType::Infinite => NormEnum::Infinite,
    NormType::L1 => NormEnum::L1,
    NormType::L2 => NormEnum::L2,
  }
}

fn build_job_output(job: &Job, system: &SystemEnum) -> Result<JobOutput, WorkerError> {

  let mut job_output: JobOutput = JobOutput { 
    is_gns: None,
    signature: None,
    all_loops: None,
    path: None,
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
      let loop_sizes: Vec<usize> = all_loops.iter().map(|loop_| loop_.len()).collect();
      let mut signature = vec![0; *loop_sizes.iter().max().unwrap_or(&0)];
      for size in &loop_sizes {
        signature[size - 1] += 1;
      }
      job_output.is_gns = Some(signature.len() == 1 && signature[0] == 1);
      job_output.signature = Some(signature);
      job_output.all_loops = Some(all_loops);
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
    JobType::Walk => {
      println!("Starting walk operation");
      let start_point = match &job.walk_from {
        Some(point) => point,
        None => {
          return Err(WorkerError::InvalidInput("Walk job without starting point".to_string()));
        }
      };
      let res = match walk(system, build_na_vector(&start_point)) {
        Ok(res) => res,
        Err(err) => {
          return Err(WorkerError::Operation(err.to_string()))
        }
      };
      job_output.path = Some(res);
    }
  };

  Ok(job_output)
}
