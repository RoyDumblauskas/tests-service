use dioxus::prelude::*;

#[server]
pub async fn get_status(url: String) -> Result<String, ServerFnError> {
   Ok(url) 
}
