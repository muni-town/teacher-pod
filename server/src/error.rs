use axum::{response::IntoResponse, http::StatusCode, response::Json};

use crate::api::OperResult;

#[allow(dead_code)]
pub enum AppError {
    MissingParams(String),
    AccessDenied,
    Sqlx(sqlx::Error),
    Custom((StatusCode, String))  
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        AppError::Sqlx(inner)
    }
}


impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::MissingParams(p) => {
                (StatusCode::BAD_REQUEST, format!("missing required parameters: {}", p))
            }
            AppError::AccessDenied => {
                (StatusCode::UNAUTHORIZED, "access denied".to_string())
            }
            AppError::Sqlx(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            },
            AppError::Custom(v) => v,
        };
        (status, Json(OperResult::err(status, &error_message))).into_response()
    }
}