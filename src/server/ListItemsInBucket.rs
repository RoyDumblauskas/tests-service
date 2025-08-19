use dioxus::prelude::*;
use std::env;

#[server]
pub async fn list_items_in_bucket() -> Result<Vec<String>, ServerFnError> {

    // declare inside server fn to get deps
    use aws_sdk_s3;
    use aws_config::BehaviorVersion;
    
    // Use APP_ENV as the bucket name
    let bucket_name = env::var("BUCKET").unwrap_or_else(|_| "DEV".to_string()).to_lowercase();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("us-east-1")
        .endpoint_url("https://imgs.roypository.com")
        .load()
        .await;

    let s3 = aws_sdk_s3::Client::new(&config);

    // List objects in bucket
    let mut itemsInBucket = s3
        .list_objects_v2()
        .bucket(bucket_name.to_owned())
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

