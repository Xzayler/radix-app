use crate::{db::db};
use std::{env, error::Error, fmt};

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
			Err(_) => DEFAULT_POLLING_TIMEOUT_SECONDS
		}
		Err(_) => DEFAULT_POLLING_TIMEOUT_SECONDS
	};


  loop {
    let pending_job = match db::pick_pending_job(&pool).await {
      Ok(res) => res,
      Err(err) => {
        return Err(SupervisorError::Database(err.to_string()));
      }
    };
    println!("{:?}", pending_job);
    match pending_job {
      Some(job) => {
        let id = job.id;
        println!("Found job with id: {}", id);
        let mut child = match std::process::Command::new(current_path)
          .arg("worker")
          .arg(id.to_string())
          .spawn() {
            Ok(res) => res,
            Err(_err) => {
              return Err(SupervisorError::ChildError("Couldn't spawn child".to_string()));
            }
          };

        let status = match child.wait() {
          // TODO error if status != 0
          Ok(status) => status,
          Err(err) => {
            return Err(SupervisorError::ChildError(err.to_string()))
          }
        };

        if !status.success() {
          let code = status.code().unwrap_or(255);
          println!("\nJob processing failed with code {}!", code);
          return Err(SupervisorError::ChildCrashed(code));
          // TODO: Update job as FAILED
        } else {
          println!("\nWorker exited successfully");
        }
      },
      None => {
        println!("Sleeping");
        // TODO: Use configuration data to set polling frequency
        tokio::time::sleep(std::time::Duration::from_secs(polling_timeout)).await;
      }
    }
  }
}

#[derive(Debug)]
pub enum SupervisorError {
  ChildError(String),
  ChildCrashed(i32),
  Database(String)
}

impl fmt::Display for SupervisorError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChildError(msg) => write!(f, "Error spawning worker: {}", msg),
      Self::ChildCrashed(code) => write!(f, "Child crashed with code {}", code),
      Self::Database(msg) => write!(f, "There was an error communicating with the database: {}", msg)
    }
  }
}

impl Error for SupervisorError {}