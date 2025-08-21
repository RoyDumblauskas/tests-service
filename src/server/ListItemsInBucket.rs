use dioxus::prelude::*;
use std::env;

#[server]
pub async fn list_items_in_bucket() -> Result<Vec<String>, ServerFnError> {

    // declare inside server fn to get deps
    use aws_sdk_s3::config::{Credentials,Region};
    use aws_sdk_config::config::BehaviorVersion;
    
    // Use APP_ENV as the bucket name
    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());
    let bucket_user = env::var(format!("MINIO_{}_USER", env_bucket)).unwrap_or("Env Var Does not exist".to_string());;
    let bucket_pass = env::var(format!("MINIO_{}_PASSWORD", env_bucket)).unwrap_or("Env Var Does not exist".to_string());
    let creds = Credentials::new(bucket_user, bucket_pass, None, None, "custome creds");

    let client_conf = aws_sdk_s3::config::Builder::new()
        .behavior_version_latest()
        .credentials_provider(creds)
        .endpoint_url("https://imgs.roypository.com")
        .region(Region::new("us-east-1"))
        .force_path_style(true)
        .build();


    let s3 = aws_sdk_s3::Client::from_conf(client_conf);

    // List objects in bucket
    let mut itemsInBucket = s3
        .list_objects_v2()
        .bucket(env_bucket.to_lowercase().to_owned())
        .max_keys(10)
        .into_paginator()
        .send();

    let mut results: Vec<String> = vec![];

    while let Some(result) = itemsInBucket.next().await {
        match result {
            Ok(output) => {
                for object in output.contents() {
                    results.push(format!(" - {}", object.key().unwrap_or("Unknown")));
                }
            }
            Err(err) => {
                eprintln!("{err:?}")
            }
        }
    }

    Ok(results)

}

