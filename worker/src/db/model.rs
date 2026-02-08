use std::option::Option;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "status")]
pub enum JobStatus {
  Pending,
  Running,
  Completed,
  Failed
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "job_type")]
pub enum JobType {
  Walk,
  Decision,
  Classification
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "input_type")]
pub enum JobInputType {
  Custom,
  Canonical,
  JCanonical,
  Dense,
  Adjoined,
  Symmetric,
  Shifted
}

#[derive(sqlx::FromRow)]
pub struct Job {
  pub id: i32,
  pub user_id: i32,
  pub status: JobStatus,
  pub job_type: JobType,
  pub input_type: JobInputType,
  pub input_uri: String,
  pub output_uri: Option<String>,
  pub created_at: Option<DateTime<Utc>>,
  pub started_at: Option<DateTime<Utc>>,
  pub finished_at: Option<DateTime<Utc>>
}