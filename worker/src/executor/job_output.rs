use nalgebra::DVector;

use crate::{
  db::model::{Job, JobType},
  error::WorkerError,
  executor::algorithm::{
    operations::{classification, decision, walk},
    systems::SystemEnum,
  },
};

#[derive(Debug)]
pub(super) struct JobOutput {
  pub(super) is_gns: Option<bool>,
  pub(super) signature: Option<Vec<i32>>,
  pub(super) all_loops: Option<Vec<Vec<DVector<f64>>>>,
  pub(super) path: Option<Vec<DVector<f64>>>,
}

pub(super) fn build_job_output(job: &Job, system: &SystemEnum) -> Result<JobOutput, WorkerError> {
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
    }
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
    }
    JobType::Walk => {
      println!("Starting walk operation");
      let start_point = match &job.walk_from {
        Some(point) => point,
        None => {
          return Err(WorkerError::InvalidInput(
            "Walk job without starting point".to_string(),
          ));
        }
      };
      let res = match walk(system, build_na_vector(start_point)) {
        Ok(res) => res,
        Err(err) => return Err(WorkerError::Operation(err.to_string())),
      };
      job_output.path = Some(res);
    }
  };

  Ok(job_output)
}

fn build_na_vector(vec: &[i32]) -> DVector<f64> {
  let v: Vec<f64> = vec.iter().map(|e| *e as f64).collect();
  DVector::from_vec(v)
}
