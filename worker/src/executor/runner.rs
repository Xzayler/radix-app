use sqlx::PgPool;

use crate::{
  db::{self, update_db_with_job_error},
  error::WorkerError,
  minio::{upload_loop_results, upload_path_result},
};

use super::{
  job_output::{JobOutput, build_job_output},
  system_builder::build_system,
};

pub async fn run(job_id: i32) {
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

  let system = match build_system(&job.system, &job.norm) {
    Ok(system) => system,
    Err(err) => {
      persist_job_error(&pool, job_id, err).await;
      return;
    }
  };
  let output = match build_job_output(&job, &system) {
    Ok(output) => output,
    Err(err) => {
      persist_job_error(&pool, job_id, err).await;
      return;
    }
  };

  let output_uri = match upload_output(job_id, &output).await {
    Ok(uri) => uri,
    Err(err) => {
      panic!("Couldn't upload file. {err}")
    }
  };

  match db::update_db_with_results(
    &pool,
    job_id,
    job.system.id,
    output.is_gns,
    output.signature,
    output_uri,
  )
  .await
  {
    Ok(_) => (),
    Err(err) => {
      panic!("Can't update db with results. {err}");
    }
  }
}

async fn persist_job_error(pool: &PgPool, job_id: i32, err: WorkerError) {
  let res = update_db_with_job_error(pool, job_id, err.to_string()).await;
  if let Err(err) = res {
    panic!("Couldn't update job {job_id} with error {err}")
  }
}

async fn upload_output(job_id: i32, output: &JobOutput) -> Result<Option<String>, WorkerError> {
  if let Some(loops) = &output.all_loops {
    println!("Uploading to minio.");
    return upload_loop_results(job_id, loops).await.map(Some);
  }

  if let Some(path) = &output.path {
    println!("Uploading to minio");
    return upload_path_result(job_id, path).await.map(Some);
  }

  Ok(None)
}
