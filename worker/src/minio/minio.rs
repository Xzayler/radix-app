use std::env;
use std::path::PathBuf;
use aws_sdk_s3::{Client, config::{Credentials, Region}};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use nalgebra::DVector;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

use crate::error::WorkerError;

pub async fn create_client() -> Result<Client,WorkerError> {
  let minio_endpoint = env::var("MINIO_ENDPOINT").map_err(|err| WorkerError::Environment("MINIO_ENDPOINT must be set: ".to_string() + err.to_string().as_str()))?;
  let minio_port = env::var("MINIO_PORT").map_err(|err| WorkerError::Environment("MINIO_PORT must be set: ".to_string() + err.to_string().as_str()))?;
  let minio_url = "http://".to_string() + minio_endpoint.as_str() + ":" + minio_port.as_str();
  
  let minio_user = env::var("MINIO_USER").map_err(|err| WorkerError::Environment("MINIO_USER must be set: ".to_string() + err.to_string().as_str()))?;
  let minio_pass = env::var("MINIO_PASSWORD").map_err(|err| WorkerError::Environment("MINIO_PASSWORD must be set: ".to_string() + err.to_string().as_str()))?;
  let credentials = Credentials::new(minio_user, minio_pass, None, None, "loaded-from-custom-env");

  let region_provider = RegionProviderChain::default_provider()
    .or_else(Region::new("us-east-1"));

  let config = aws_config::from_env()
    .region(region_provider)
    .endpoint_url(minio_url)
    .credentials_provider(credentials)
    .load()
    .await;

  let s3_config = aws_sdk_s3::config::Builder::from(&config)
    .force_path_style(true)
    .build();

  Ok(Client::from_conf(s3_config))
}

pub async fn upload_job_results(job_id: i32, loops: &Vec<Vec<DVector<f64>>>) -> Result<String, WorkerError> {
  let client = create_client().await?;
  let bucket = env::var("MINIO_BUCKET").map_err(|err| WorkerError::Environment("MINIO_BUCKET must be set: ".to_string() + err.to_string().as_str()))?;
  let key = format!("{job_id}.json");
  let temp_path = temp_output_path(job_id)?;

  let upload_result = async {
    let mut file = File::create(&temp_path)
      .await
      .map_err(|err| WorkerError::Minio(format!("Failed to create temp file: {err}")))?;

    write_loops_json(&mut file, &loops).await?;
    file
      .flush()
      .await
      .map_err(|err| WorkerError::Minio(format!("Failed to flush temp file: {err}")))?;
    drop(file);

    let body = ByteStream::from_path(&temp_path)
      .await
      .map_err(|err| WorkerError::Minio(format!("Failed to read temp file for upload: {err}")))?;

    client
      .put_object()
      .bucket(bucket)
      .key(&key)
      .content_type("application/json")
      .body(body)
      .send()
      .await
      .map_err(|err| 
        WorkerError::Minio(format!("Failed to upload job results: {err}"))
      )?;

    Ok::<(), WorkerError>(())
  }
  .await;

  let cleanup_result = fs::remove_file(&temp_path).await;
  if let Err(err) = cleanup_result {
    println!("Failed to clean up temp file: {err}");
  }

  upload_result?;
  Ok(key)
}

fn temp_output_path(job_id: i32) -> Result<PathBuf, WorkerError> {
  Ok(std::env::temp_dir().join(format!(
    "radix-job-{job_id}.json"
  )))
}

async fn write_loops_json(file: &mut File, loops: &[Vec<DVector<f64>>]) -> Result<(), WorkerError> {
  write_json_bytes(file, b"[").await?;
  for (loop_index, loop_points) in loops.iter().enumerate() {
    if loop_index > 0 {
      write_json_bytes(file, b",").await?;
    }
    write_json_bytes(file, b"[").await?;

    for (point_index, point) in loop_points.iter().enumerate() {
      if point_index > 0 {
        write_json_bytes(file, b",").await?;
      }
      write_dvector_json(file, point).await?;
    }

    write_json_bytes(file, b"]").await?;
  }
  write_json_bytes(file, b"]").await
}

async fn write_dvector_json(file: &mut File, vector: &DVector<f64>) -> Result<(), WorkerError> {
  write_json_bytes(file, b"[").await?;
  for (value_index, value) in vector.iter().enumerate() {
    if value_index > 0 {
      write_json_bytes(file, b",").await?;
    }

    let value_text = (*value as i64).to_string();
    write_json_bytes(file, value_text.as_bytes()).await?;
  }
  write_json_bytes(file, b"]").await
}

async fn write_json_bytes(file: &mut File, bytes: &[u8]) -> Result<(), WorkerError> {
  file
    .write_all(bytes)
    .await
    .map_err(|err| WorkerError::Minio(format!("Failed to write JSON output: {err}")))
}
