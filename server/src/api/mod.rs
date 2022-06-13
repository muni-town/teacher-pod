use salvo::{
    http::HeaderValue,
    hyper::{header::CONTENT_TYPE, StatusCode},
};
use serde::Serialize;

pub mod account;

#[derive(Serialize, Debug)]
pub struct ApiData<T: Serialize> {
    pub code: u16,
    pub data: T,
    pub message: String,
}

pub trait JsonApi {
    fn api<T: Serialize>(&mut self, code: StatusCode, data: T, message: &str);
    fn success<T: Serialize>(&mut self, data: T);
}

impl JsonApi for salvo::Response {
    fn api<T: Serialize>(&mut self, code: StatusCode, data: T, message: &str) {
        self.headers_mut().append(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        self.render(
            serde_json::to_string(&ApiData {
                code: code.as_u16(),
                data,
                message: message.to_string(),
            })
            .unwrap(),
        );
    }

    fn success<T: Serialize>(&mut self, data: T) {
        self.headers_mut().append(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        self.render(
            serde_json::to_string(&ApiData {
                code: 200,
                data,
                message: String::new(),
            })
            .unwrap(),
        );
    }
}
