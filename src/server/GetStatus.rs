use dioxus::prelude::*;

#[server(GetStatus)]
pub async fn get_status(input: String) -> Result<String, ServerFnError> {
   Ok(input) 
}
