use std::{env, error::Error, fmt, os::unix::process::CommandExt};

use crate::db::{self, update_db_with_job_error};

pub async fn run(current_path: &str) -> Result<(), SupervisorError> {
  let pool = match db::connect().await {
    Ok(pool) => pool,
    Err(err) => {
      return Err(SupervisorError::Database(err.to_string()));
    }
  };
  const DEFAULT_POLLING_TIMEOUT_SECONDS: u64 = 10;
  let polling_timeout = match env::var("POLLING_RATE") {
    Ok(str_value) => match str::parse::<u64>(str_value.as_str()) {
      Ok(value) => value,
      Err(_) => DEFAULT_POLLING_TIMEOUT_SECONDS,
    },
    Err(_) => DEFAULT_POLLING_TIMEOUT_SECONDS,
  };

  loop {
    println!("Polling for jobs");
    let pending_job = match db::pick_pending_job(&pool).await {
      Ok(res) => res,
      Err(err) => {
        return Err(SupervisorError::Database(err.to_string()));
      }
    };

    match pending_job {
      Some(job) => {
        let id = job.id;
        println!("Found job with id: {}", id);
        let process = unsafe {
          std::process::Command::new(current_path)
            .pre_exec(|| {
              let pid = std::process::id();
              std::fs::write(
                  format!("/proc/{}/oom_score_adj", pid),
                  "1000\n"
                )?;
                Ok(())
              })
            .arg("worker")
            .arg(id.to_string())
            .spawn()
          };
        let mut child = match process {
          Ok(res) => res,
          Err(_err) => {
            return Err(SupervisorError::ChildError(
              "Couldn't spawn child".to_string(),
            ));
          }
        };

        let status = match child.wait() {
          Ok(status) => status,
          Err(err) => return Err(SupervisorError::ChildError(err.to_string())),
        };

        if !status.success() {
          let code = status.code().unwrap_or(255);
          println!("\nJob processing failed with code {}!", code);
          let error_message = match code {
            255 => "Process aborted",
            _ => "Process error"
          };
          if let Err(err) = update_db_with_job_error(&pool, id, error_message.to_string() + ". The server might not have enough resources to process the job or infrastructure might be unreachable.").await {
            return Err(SupervisorError::Database(format!("Couldn't update job {id} with error {err}")))
          }
        } else {
          println!("\nWorker exited successfully");
        }
      }
      None => {
        tokio::time::sleep(std::time::Duration::from_secs(polling_timeout)).await;
      }
    }
  }
}

#[derive(Debug)]
pub enum SupervisorError {
  ChildError(String),
  Database(String),
}

impl fmt::Display for SupervisorError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChildError(msg) => write!(f, "Error spawning worker: {}", msg),
      Self::Database(msg) => write!(
        f,
        "There was an error communicating with the database: {}",
        msg
      ),
    }
  }
}

impl Error for SupervisorError {}
