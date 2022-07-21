use salvo::{
    async_trait, hyper::StatusCode, prelude::StatusError, Depot, Request, Response, Writer,
};

use crate::api::JsonApi;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    QueryNotFound(String),
    DataNotFound,
    AuthorizationFailed(String),
    Unauthorized,
    DataExists,
}

pub type ApiResult = Result<(), Error>;

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let info: (String, StatusCode) = match self {
            Error::Database(e) => (e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            Error::QueryNotFound(s) => {
                let message = format!("query `{}` not found", s);
                (message, StatusCode::BAD_REQUEST)
            }
            Error::DataNotFound => {
                let message = format!("data not found");
                (message, StatusCode::BAD_REQUEST)
            }
            Error::AuthorizationFailed(s) => (
                format!("account authentication failed: {}", s),
                StatusCode::BAD_REQUEST,
            ),
            Error::Unauthorized => ("unauthorized".into(), StatusCode::UNAUTHORIZED),
            Error::DataExists => ("data exists".into(), StatusCode::BAD_REQUEST),
        };
        res.set_status_error(StatusError::from_code(info.1).unwrap());
        res.api::<String>(
            info.1,
            StatusError::from_code(info.1).unwrap().name,
            &info.0,
        );
    }
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database(inner)
    }
}
