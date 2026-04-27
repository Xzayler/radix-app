use core::panic;
use std::option::Option;
use sqlx::{FromRow, Row, postgres::PgRow};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "digit_type", rename_all = "PascalCase")]
pub enum DigitType {
  Explicit,
  Canonical,
  JCanonical,
  Adjoint,
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
  // digit fields
  pub digit_param: Option<i32>,
  pub digits: Option<Vec<Vec<i32>>>
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
  pub system: DbSystem,
  pub norm: NormType,
  pub job_type: JobType,
  pub walk_from: Option<Vec<i32>>
}

impl FromRow<'_, PgRow> for Job {
  fn from_row(row: &PgRow) -> sqlx::Result<Self> {
    let digits: Option<Vec<Vec<i32>>> = match row.get::<Option<String>, &str>("digits") {
      Some(str_value) => Some(parse_vector_of_vectors(&str_value)),
      None => None,
    };

    let system = DbSystem {
      id: row.get("system_id"),
      dimension: row.get("dimension"),
      base: row.get("base"),
      digit_type: row.get("digit_type"),
      // digit fields
      digit_param: row.get("digit_param"),
      digits: digits
    };
    Ok(Self {
      system: system,
      job_type: row.get("job_type"),
      norm: row.get("norm"),
      walk_from: row.get("start_point")
    })
  }
}

pub fn parse_vector_of_vectors(s: &str) -> Vec<Vec<i32>> {
  let inner = &s[1..s.len() - 1];
  let parts: Vec<&str> = inner.split("},{").collect();
  let mut result = Vec::new();
  for part in parts {
    let cleaned = part.trim_matches('{').trim_matches('}');
    let nums: Vec<i32> = cleaned
      .split(',')
      .map(|n| match n.trim().parse::<i32>() {
        Ok(res) => res,
        Err(err) => panic!("Could not parse digits from database! {err}"),
      })
      .collect();
    result.push(nums);
  }
  result
}

#[derive(Debug, sqlx::FromRow)]
pub struct JobId {
  pub id: i32,
}
