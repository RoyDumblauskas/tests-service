use dioxus::prelude::*;
use std::env;

#[server]
pub async fn get_status(url: String) -> Result<String, ServerFnError> {
    // let key = env::var("KEY").unwrap_or("API key does not exist in environment".to_string()); 
    
    match env::var("ENV_BUCKET") {
        Ok(val) => Ok(val),
        Err(e) => Ok(e.to_string()),
    }
}
