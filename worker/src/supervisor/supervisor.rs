use crate::{db::db, executor::executor::WorkerError};
use std::env;

pub async fn run(current_path: &str) -> Result<(), WorkerError> {
  let pool = match db::connect().await {
    Ok(pool) => pool,
    Err(err) => {
      return Err(WorkerError::Database(err.to_string()));
    }
  };
	const DEFAULT_POLLING_TIMEOUT: u64 = 10;
	let polling_timeout = match env::var("POLLING_RATE") {
		Ok(str_value) => match str::parse::<u64>(str_value.as_str()) {
			Ok(value) => value,
			Err(_) => DEFAULT_POLLING_TIMEOUT
		}
		Err(_) => DEFAULT_POLLING_TIMEOUT
	};


  loop {
    let pending_job = match db::pick_pending_job(&pool).await {
      Ok(res) => res,
      Err(err) => {
        return Err(WorkerError::Database(err.to_string()));
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
            Err(err) => {
              return Err(WorkerError::Unhandled("Couldn't spawn child".to_string()));
            }
          };

        let status = match child.wait() {
          Ok(status) => status,
          Err(err) => {
            return Err(WorkerError::Unhandled("Child not running".into()))
          }
        };

        if !status.success() {
          // TODO: Update job as FAILED
          println!("\nJob processing failed!");
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