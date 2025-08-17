use aws_sdk_s3::{Client, Config, Endpoint, Region};
use aws_types::credentials::Credentials;
use aws_smithy_http::endpoint::Endpoint as SmithyEndpoint;
use std::str::FromStr;

/// Helper function to build an S3 client for MinIO
fn minio_client() -> Result<Client, ServerFnError> {
    // Determine environment (DEV, PROD, etc.)
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "DEV".to_string());

    // Dynamically construct the env var names for user/pass
    let user_var = format!("MINIO_{}_USER", app_env);
    let pass_var = format!("MINIO_{}_PASS", app_env);

    let user = env::var(&user_var)
        .map_err(|_| ServerFnError::ServerError(format!("Missing env var: {}", user_var)))?;
    let pass = env::var(&pass_var)
        .map_err(|_| ServerFnError::ServerError(format!("Missing env var: {}", pass_var)))?;

    // Construct AWS credentials
    let credentials = Credentials::new(&user, &pass, None, None, "minio-credentials");

    // Endpoint for your MinIO instance
    let endpoint = Endpoint::immutable(
        smithy_http::endpoint::Uri::from_str("https://imgs.roypository.com")
            .map_err(|e| ServerFnError::ServerError(format!("Invalid endpoint URL: {}", e)))?,
    );

    // Build the client config
    let config = Config::builder()
        .region(Region::new("us-east-1")) // MinIO ignores this
        .endpoint_resolver(endpoint)
        .credentials_provider(credentials)
        .build();

    Ok(Client::from_conf(config))
}
