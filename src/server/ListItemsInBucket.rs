use dioxus::prelude::*;
use std::env;

#[server]
pub async fn list_items_in_bucket() -> Result<String, ServerFnError> {
    let client = minio_client()?; // get the configured client

    // Use APP_ENV as the bucket name
    let bucket_name = env::var("APP_ENV").unwrap_or_else(|_| "DEV".to_string()).to_lowercase();

    // List objects in bucket
    match client.list_objects_v2().bucket(bucket_name.clone()).send().await {
        Ok(resp) => {
            let keys: Vec<String> = resp
                .contents
                .unwrap_or_default()
                .into_iter()
                .filter_map(|obj| obj.key)
                .collect();
            Ok(format!("Objects in bucket '{}': {:?}", bucket_name, keys))
        }
        Err(e) => Ok(format!("Failed to list objects: {}", e)),
    }
}

