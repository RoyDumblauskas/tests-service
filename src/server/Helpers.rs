use dioxus::prelude::*;
use aws_sdk_s3;
use aws_sdk_config;
use aws_config;
use std::env;


/// Helper function to build an S3 client for MinIO
#[server]
pub async fn minio_client() -> Result<String, ServerFnError> {
    let bucket = env::var("BUCKET").unwrap_or("DEV".to_string());

    let config = aws_config::l;
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .endpoint_url("imgs.roypository.com")
        .build();

    // Ok(s3 = aws_sdk_s3::Client::from_conf(s3_config));
    Ok("hello".to_string())
}
