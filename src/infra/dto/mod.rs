use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub url: Option<String>,
    pub error_msg: Option<String>,
}

impl RegisterResponse {
    pub fn new(url: Option<String>, error_msg: Option<String>) -> Self {
        Self { url, error_msg }
    }
}
