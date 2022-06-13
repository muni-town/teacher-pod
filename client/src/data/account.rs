use crate::hooks::use_storage;

use super::{
    model::{AuthInfo, ApiData, SimpleUser},
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

    let data = resp.json::<ApiData<AuthInfo>>().await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(match data.message.as_str() {
            "data not found" => "User Info Not Found".to_string(),
            _ => data.message,
        }));
    }

    let storage = use_storage()?;
    storage.set_item("auth", &data.data.token).unwrap();

    return Ok(());
}
