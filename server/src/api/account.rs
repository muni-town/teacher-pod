use std::iter::repeat_with;

use salvo::prelude::*;

use crate::{
    auth,
    error::{ApiResult, Error},
    models::account::{Account, Auth},
    Routers,
};

use super::{block_unlogin, JsonApi};

#[fn_handler]
async fn current_account(depot: &mut Depot, resp: &mut Response) -> ApiResult {
    let user = depot.get::<Account>("user-info");
    if user.is_none() {
        return Err(Error::Unauthorized);
    }
    resp.success(user.unwrap());
    Ok(())
}

#[fn_handler]
async fn login(req: &mut Request, resp: &mut Response) -> ApiResult {
    let email = req.query::<String>("email");
    let password = req.query::<String>("password");
    if email.is_none() {
        return Err(Error::QueryNotFound("email".into()));
    }
    if password.is_none() {
        return Err(Error::QueryNotFound("password".into()));
    }
    let email = email.unwrap();
    let password = password.unwrap();

    let user = Account::query_from_email(&email).await?;

    let checker = Account::check_password(&password, &user.password, &user.salt);
    if checker {
        let now = chrono::Local::now().timestamp() as i32;
        let expire = now + 60 * 60 * 24 * 2;
        let auth_id: String = repeat_with(fastrand::alphanumeric).take(12).collect();
        let token = auth::encode(&auth::AuthClaims {
            exp: expire,
            iat: now,
            id: auth_id.clone(),
            user: user.id,
        });
        Auth::insert_auth_info(&auth_id, user.id, expire).await?;
        resp.success(serde_json::json!({
            "token": token,
            "expire": expire,
        }));
    } else {
        resp.api::<String>(StatusCode::BAD_REQUEST, String::new(), "data not found");
    }

    Ok(())
}

pub struct AccountApi;
impl Routers for AccountApi {
    fn build() -> Vec<salvo::Router> {
        vec![
            Router::new().path("/login").handle(login),
            Router::new()
                .path("self")
                .hoop(block_unlogin)
                .get(current_account),
        ]
    }
}
