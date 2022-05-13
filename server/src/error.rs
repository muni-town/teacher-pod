use axum::{response::IntoResponse, http::StatusCode, response::Json};

pub enum AppError {
    MissingParams(String),
    Sqlx(sqlx::Error)    
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
                (StatusCode::BAD_REQUEST, format!("Missing required parameters: {}", p))
            }
            AppError::Sqlx(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            },
        };

        Json(serde_json::json!({
            "code": status.to_string(),
            "message": error_message,
        })).into_response()
    }
}