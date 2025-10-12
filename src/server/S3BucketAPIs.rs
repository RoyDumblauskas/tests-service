use dioxus::prelude::*;
use std::env;

#[server]
pub async fn list_items_in_bucket() -> Result<Vec<String>, ServerFnError> {
    use crate::server::ServerUtils::server_utils::get_s3_client;
    let s3 = get_s3_client().await.unwrap();

    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());

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

#[server]
pub async fn put_item_in_bucket(key: String, object: String) -> Result<i32, ServerFnError> {
    use crate::server::ServerUtils::server_utils::get_s3_client;
    let s3 = get_s3_client().await.unwrap();

    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());

    let body = aws_sdk_s3::primitives::ByteStream::from(object.clone().into_bytes());

    let response = s3
        .put_object()
        .bucket(env_bucket.to_lowercase().to_owned())
        .key(key.clone())
        .body(body)
        .send()
        .await;

    match response {
        Ok(output) => {
            Ok(0) 
        }
        Err(err) => {
            eprintln!("{err:?}");
            Ok(-1) 
        }
    }
}

#[server]
pub async fn get_item_in_bucket(key: String) -> Result<String, ServerFnError> {
    use crate::server::ServerUtils::server_utils::get_s3_client;
    let s3 = get_s3_client().await.unwrap();

    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());

    let item = s3
        .get_object()
        .bucket(env_bucket.to_lowercase().to_owned())
        .key(key.clone())
        .send()
        .await;

    match item {
        Ok(item) => {
            let bytes = item.body.collect().await?.into_bytes();
            Ok(String::from_utf8(bytes.to_vec())?)
        }
        Err(err) => {
            Ok(format!("Error: {}", err.to_string()))
        }
    }
}

#[server]
pub async fn delete_item_in_bucket(key: String) -> Result<i32, ServerFnError> {
    use crate::server::ServerUtils::server_utils::get_s3_client;
    let s3 = get_s3_client().await.unwrap();

    let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());

    let resp = s3
        .delete_object()
        .bucket(env_bucket.to_lowercase().to_owned())
        .key(key.clone())
        .send()
        .await;

    match resp {
        Ok(output) => {
            Ok(0)
        }
        Err(err) => {
            eprintln!("{err:?}");
            Ok(-1) 
        }
    }
}
