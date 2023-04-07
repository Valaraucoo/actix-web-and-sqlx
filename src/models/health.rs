use gethostname::gethostname;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub hostname: String,
    pub timestamp: i64,
}

impl HealthResponse {
    pub async fn new() -> Self {
        Self {
            status: "ok".to_string(),
            hostname: gethostname().into_string().unwrap(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}
