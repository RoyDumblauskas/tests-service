use dioxus::prelude::*;
use std::env;

#[server]
pub async fn list_items_in_bucket() -> Result<Vec<String>, ServerFnError> {
    use crate::server_utils::get_s3_client;

    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());

    let s3 = get_s3_client().await.unwrap();

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
                    results.push(format!(" - {}", object.key().unwrap_or("Bleep Bloop Error")));
                }
            }
            Err(err) => {
                eprintln!("{err:?}")
            }
        }
    }

    Ok(results)

}


