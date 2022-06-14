use salvo::{http::HeaderValue, hyper::header::CONTENT_TYPE, prelude::*};
use serde::Serialize;

use crate::{
    auth,
    error::{ApiResult, Error},
    models::account::{Account, Auth},
};

pub mod account;
pub mod podcast;

#[fn_handler]
pub async fn block_unlogin(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> ApiResult {
    let token = req.header::<String>("Authorization");
    if token.is_none() {
        return Err(Error::AuthorizationFailed("authorization not found".into()));
    }
    let token = token.unwrap();

    if !token.starts_with("Bearer ") {
        return Err(Error::AuthorizationFailed("bearer not found".into()));
    }
    let token = token[7..token.len()].to_string();

    let claims = auth::decode(&token);
    if claims.is_none() {
        return Err(Error::AuthorizationFailed("decode failed".into()));
    }
    let claims = claims.unwrap();

    if !Auth::check_auth_info(&claims.id, claims.user).await {
        return Err(Error::AuthorizationFailed("check auth failed".into()));
    }

    let user = Account::query_from_id(claims.user).await;
    if user.is_err() {
        return Err(Error::AuthorizationFailed("query user failed".into()));
    }
    let user = user.unwrap();
    depot.insert("user-info", user);

    ctrl.call_next(req, depot, res).await;

    Ok(())
}

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
