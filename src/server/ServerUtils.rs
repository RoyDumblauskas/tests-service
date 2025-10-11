#[cfg(server)]
pub mod server_utils {
    use aws_sdk_s3::Client;
    use aws_config;
    use dioxus::prelude::*;


    pub async fn get_s3_client() -> Result<aws_sdk_s3, ServerFnError> {

        // declare inside server fn to get deps
        use aws_sdk_s3::config::{Credentials,Region};
        
        // Use APP_ENV as the bucket name
        let env_bucket= env::var("ENV_BUCKET").unwrap_or("Env Var Does not exist".to_string());
        let bucket_user = env::var(format!("MINIO_{}_USER", env_bucket)).unwrap_or("Env Var Does not exist".to_string());;
        let bucket_pass = env::var(format!("MINIO_{}_PASSWORD", env_bucket)).unwrap_or("Env Var Does not exist".to_string());
        let creds = Credentials::new(bucket_user, bucket_pass, None, None, "custom creds");

        let client_conf = aws_sdk_s3::config::Builder::new()
            .behavior_version_latest()
            .credentials_provider(creds)
            .endpoint_url("https://imgs.roypository.com")
            .region(Region::new("us-east-1"))
            .force_path_style(true)
            .build();

        Ok(aws_sdk_s3::Client::from_conf(client_conf))
    }
}
