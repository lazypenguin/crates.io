use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub s3_bucket: String,
    pub s3_region: Option<String>,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_proxy: Option<String>,
    pub session_key: String,
    pub git_repo_checkout: PathBuf,
    pub gh_client_id: String,
    pub gh_client_secret: String,
    pub db_url: String,
    pub env: ::Env,
    pub max_upload_size: u64,
}

impl Config {
    pub fn api_protocol(&self) -> &'static str {
        // When testing we route all API traffic over HTTP so we can
        // sniff/record it, but everywhere else we use https
        if self.env == ::Env::Test {"http"} else {"https"}
    }
}
