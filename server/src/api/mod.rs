use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub mod users;
pub mod account;
pub mod contents;
pub mod topics;


#[derive(Serialize, Deserialize)]
pub struct OperResult {
    status: String,
    message: String,
}

impl OperResult {
    pub fn ok() -> Self {
        Self {
            status: StatusCode::OK.to_string(),
            message: String::new(),
        }
    }
    pub fn err(status: StatusCode, msg: &str) -> Self {
        Self {
            status: status.to_string(),
            message: msg.into(),
        }
    }
}
