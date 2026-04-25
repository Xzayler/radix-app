mod executor;
mod supervisor;
mod db;
mod minio;
mod error;
use std::{error::Error, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = std::env::args().collect();
  
  match args.get(1).map(String::as_str) {
    Some("worker") => {
      println!("Attempting to start worker");
      let id = match args.get(2) {
        Some(res) => {
          match res.parse::<i32>() {
            Ok(parsed) => parsed,
            Err(_) => {
              println!("Can't parse job id for worker");
              process::exit(1);
            }
          }
        },
        None => {
          println!("Missing job id for worker.");
          process::exit(1);
        }
      };
      executor::run(id).await;
    
      process::exit(0);
    }
    _ => {
      println!("Starting supervisor.");
      let curr = match args.get(0) {
        Some(curr) => curr,
        None => {
          println!("Couldn't get path arg");
          process::exit(1);
        }
      };
      match supervisor::run(curr).await {
        Ok(_) => (),
        Err(err) => {
          println!("Couldn't start worker: {err}");
          process::exit(1);
        }
      }
    }
  };
  
  Ok(())
}
