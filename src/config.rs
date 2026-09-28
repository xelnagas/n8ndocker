use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub bionic_base_url: String,
    pub bionic_api_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: env::var("RUST_TOOLS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("RUST_TOOLS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            bionic_base_url: env::var("BIONIC_BASE_URL")
                .unwrap_or_else(|_| "http://host.docker.internal:1234".to_string()),
            bionic_api_key: env::var("BIONIC_API_KEY").ok(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
}
