use super::request::get;

pub async fn is_login() -> bool {
    let resp = get("/self").await;
    resp.is_ok() && resp.unwrap().status() == 200
}