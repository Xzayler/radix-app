use std::option::Option;
use sqlx::{FromRow, Row, postgres::PgRow, types::chrono::{DateTime, Utc}};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "digit_type", rename_all = "PascalCase")]
pub enum DigitType {
  Explicit,
  Canonical,
  JCanonical,
  Dense,
  Adjoined,
  Symmetric,
  JSymmetric,
  Shifted
}

#[derive(Debug, sqlx::FromRow)]
pub struct DbSystem {
  pub id: i32,
  pub dimension: i32,
  pub base: Vec<i32>,
  pub digit_type: DigitType,
  pub is_gns: Option<bool>,
  pub signature: Option<Vec<i32>>,
  pub last_job: Option<DateTime<Utc>>,
  // digit fields
  pub digit_param: Option<i32>,
  pub digits: Option<Vec<Vec<i32>>>
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "status")]
pub enum JobStatus {
  Pending,
  Running,
  Succeeded,
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
#[sqlx(type_name = "norm_type")]
pub enum NormType {
  Infinite,
  L1,
  L2
}

#[derive(Debug)]
pub struct Job {
  pub id: i32,
  pub system: DbSystem,
  pub status: JobStatus,
  pub norm: NormType,
  pub job_type: JobType,
  pub output_uri: Option<String>,
  pub started_at: Option<DateTime<Utc>>,
  pub finished_at: Option<DateTime<Utc>>,
}

impl FromRow<'_, PgRow> for Job {
  fn from_row(row: &PgRow) -> sqlx::Result<Self> {
    
    let digits: Option<Vec<Vec<i32>>> = match row.get::<Option<String>, &str>("digits") {
      Some(str_value) => {
        Some(parse_vector_of_vectors(&str_value))
      },
      None => None
    };
    
    let system = DbSystem {
      id: row.get("system_id"),
      dimension: row.get("dimension"),
      base: row.get("base"),
      digit_type: row.get("digit_type"),
      is_gns: row.get("is_gns"),
      signature: row.get("signature"),
      last_job: row.get("last_job"),
      // digit fields
      digit_param: row.get("digit_param"),
      digits: digits
    };
    Ok(Self {
      id: row.get("id"),
      system: system,
      status: row.get("status"),
      job_type: row.get("job_type"),
      norm: row.get("norm"),
      output_uri: row.get("output_uri"),
      started_at: row.get("started_at"),
      finished_at: row.get("finished_at"),
    })
  }
}


pub fn parse_vector_of_vectors(s: &str) -> Vec<Vec<i32>> {
  // if !s.starts_with("{{") || !s.ends_with("}}") {
  //     return Err("Invalid format: string must start with {{ and end with }}".into());
  // }
  let inner = &s[1..s.len() - 1]; // remove outer {}
  let parts: Vec<&str> = inner.split("},{").collect();
  let mut result = Vec::new();
  for part in parts {
    let cleaned = part.trim_matches('{').trim_matches('}');
    let nums: Vec<i32> = cleaned.split(',')
      .map(|n| {
          match n.trim().parse::<i32>() {
            Ok(res)=> res,
            Err(_) => 99
          }
        }
      )
      .collect();
    result.push(nums);
  }
  result
}

#[derive(Debug, sqlx::FromRow)]
pub struct JobId {
  pub id: i32
}