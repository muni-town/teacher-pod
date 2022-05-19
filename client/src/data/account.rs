use crate::hooks::use_storage;

use super::{
    model::{AuthInfo, OperResult, SimpleUser},
    request::get,
};

pub fn token() -> String {
    let storage = use_storage().unwrap();
    storage.get_item("auth").unwrap().unwrap_or_default()
}

pub async fn current_user() -> Option<SimpleUser> {
    let resp = get("/self")
        .header("Authorization", &format!("Bearer {}", token()))
        .send()
        .await;
    if resp.is_err() {
        return None;
    }
    let resp = resp.unwrap();
    if !resp.ok() {
        return None;
    }
    Some(resp.json::<SimpleUser>().await.unwrap())
}

pub async fn login(email: &str, password: &str) -> anyhow::Result<()> {
    let path = format!("/login?email={email}&password={password}");
    let resp = get(&path).send().await?;
    if resp.status() != 200 {
        let message = resp.json::<OperResult>().await.unwrap().message;
        return Err(anyhow::anyhow!(match message.as_str() {
            "data not found" => "User Info Not Found".to_string(),
            _ => message,
        }));
    }

    let resp = resp.json::<AuthInfo>().await?;

    let storage = use_storage()?;
    storage.set_item("auth", &resp.token).unwrap();

    return Ok(());
}
