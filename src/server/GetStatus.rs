use dioxus::prelude::*;
use std::env;

#[server]
pub async fn get_status(url: String) -> Result<String, ServerFnError> {
    let key = env::var("KEY").unwrap_or("API key does not exist in environment".to_string()); 
    
    Ok(key) 
}
