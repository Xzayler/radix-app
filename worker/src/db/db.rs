use sqlx::{Postgres, postgres::PgPoolOptions, Pool};
use std::env;
use super::model::Job;
use super::model::JobStatus;
use super::model::JobType;
use super::model::JobInputType;

pub async fn get_job(pool: &sqlx::PgPool) -> Result<Job, sqlx::Error> {
  let job = sqlx::query_as!(
    Job,
    "SELECT
    id,
    user_id,
    status AS \"status: JobStatus\",
    job_type AS \"job_type: JobType\",
    input_type AS \"input_type: JobInputType\",
    input_uri,
    output_uri,
    created_at,
    started_at,
    finished_at
    FROM jobs
    WHERE status='Pending'",
  )
  .fetch_one(pool)
  .await
  .unwrap();

  Ok(job)
}

pub async fn connect() -> Pool<Postgres> {

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  PgPoolOptions::new()
    .max_connections(1)
    .min_connections(1)
    .connect(&database_url)
    .await
    .expect("Failed to create pool.")
}