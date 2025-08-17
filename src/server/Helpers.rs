use dioxus::prelude::*;
use aws_config;
use std::env;


/// Helper function to build an S3 client for MinIO
#[server]
pub async fn minio_client() -> Result<Client, ServerFnError> {
    let bucket = env::var("BUCKET").unwrap_or("DEV".to_string());

    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let resolver = StageResolver { stage: std::env::var("STAGE").unwrap() };

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .endpoint_resolver(resolver)
        .build();

    Ok(s3 = aws_sdk_s3::Client::from_conf(s3_config));
}
