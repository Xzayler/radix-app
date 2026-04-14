use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use crate::models::WorkerError;

use super::model::{Job, JobId};

pub async fn pick_pending_job(pool: &sqlx::PgPool) -> Result<Option<JobId>, WorkerError> {
  let job_id: Option<JobId> = sqlx::query_as::<Postgres, JobId>(
    "UPDATE jobs 
    SET
      status = 'Running',
      started_at = NOW()
    WHERE id = (
      SELECT id
      FROM jobs
      WHERE status = 'Pending'
      ORDER BY created_at ASC
      FOR UPDATE SKIP LOCKED
      LIMIT 1
    )
    RETURNING id;
  ").fetch_optional(pool)
  .await
  .map_err(|err| WorkerError::Database(format!("Couldn't pick job: {err}")))?;
  
  Ok(job_id)
}

pub async fn get_job(pool: &sqlx::PgPool, id: i32) -> Result<Job, WorkerError> {
  let job: Job = sqlx::query_as(
    format!("SELECT
    jobs.id,
    status,
    job_type,
    norm,
    output_uri,
    started_at,
    finished_at,
    systems.id as system_id,
    systems.dimension,
    systems.base,
    systems.digit_type,
    systems.is_gns,
    systems.signature,
    systems.last_job,
    systems.digit_param,
    (
      SELECT array_agg(v.elements)
      FROM digits v
      WHERE v.id = ANY(systems.digits)
    )::text as digits
    FROM jobs
    JOIN systems ON jobs.system_id = systems.id
    WHERE jobs.id = {id}").as_str()
  )
  .fetch_one(pool)
  .await
  .map_err(|err| WorkerError::Database(format!("Couldn't get job with id {id}: {err}")))?;

  Ok(job)
}

pub async fn update_db_with_results(
  pool: &sqlx::PgPool,
  job_id: i32,
  system_id: i32,
  is_gns: Option<bool>,
  signature: Option<Vec<i32>>,
  output_uri: Option<String>
) -> Result<(), WorkerError> {
  let mut transaction = pool.begin()
    .await
    .map_err(|err| WorkerError::Database("Couldn't start transaction: ".to_string() + err.to_string().as_str()))?;
  
  let set_output_uri_string = match output_uri {
    Some(uri) => format!("output_uri = '{uri}',"),
    None => "".to_string()
  };

  // TODO: Walk results
  let job_res = sqlx::query(
    format!("UPDATE jobs
    SET status = 'Succeeded',
      {set_output_uri_string}
      finished_at = NOW()
    WHERE id = {job_id}"
    ).as_str()
  )
  .execute(&mut *transaction)
  .await
  .map_err(|err| WorkerError::Database(err.to_string()))?;
  if job_res.rows_affected() == 0 {
    return Err(WorkerError::Database("Couldn't find row to update".to_string()));
  }
  
  let set_gns_string = match is_gns {
    Some(gns) => format!("is_gns = {gns},"),
    None => "".to_string()
  };

  let set_signature_string = match signature {
    Some(sig) => {
      let sig_str = vec_to_sql_string(sig);
      format!("signature = ARRAY{sig_str},")
    },
    None => "".to_string()
  };
  let system_res = sqlx::query(
    format!("UPDATE systems
    SET 
    {set_gns_string}
    {set_signature_string}
    last_job = NOW()
    WHERE id = {system_id}
    ").as_str()
  )
  .execute(&mut *transaction)
  .await
  .map_err(|err| WorkerError::Database(err.to_string()))?;
  if system_res.rows_affected() == 0 {
    return Err(WorkerError::Database("Couldn't find row to update". to_string()));
  }

  transaction.commit().await
  .map_err(|err| WorkerError::Database(format!("Couldn't update results: {err}")))?;

  Ok(())
}

pub async fn update_db_with_job_error(pool: &sqlx::PgPool, job_id: i32, err: String) -> Result<(), WorkerError> {
  
  let error_string = err.to_string();

  let res = sqlx::query(
    format!("UPDATE jobs
    SET status = 'Failed',
      error = {error_string}
      finished_at = NOW()
    WHERE id = {job_id}"
    ).as_str()
  )
  .execute(pool)
  .await
  .map_err(|err| WorkerError::Database(format!("Couldn't update failed job id {job_id}: {err}")))?;
  if res.rows_affected() == 0 {
    return Err(WorkerError::Database("Couldn't find row to update".to_string()));
  }
  Ok(())
}

pub async fn connect() -> Result<Pool<Postgres>, WorkerError> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  PgPoolOptions::new()
    .max_connections(1)
    .min_connections(1)
    .connect(&database_url)
    .await
    .map_err(|err| WorkerError::Database("Could't connect to db: ".to_string() + err.to_string().as_str()))
}

fn vec_to_sql_string(vec: Vec<i32>) -> String {
  let str_vec: Vec<String> = vec.into_iter().map(|e| e.to_string()).collect();
  "[".to_string() + &str_vec.join(",") + "]"
}