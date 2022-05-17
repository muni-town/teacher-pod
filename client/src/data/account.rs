use super::{request::get, model::OperResult};

pub async fn is_login() -> bool {
    let resp = get("/self").send().await;
    resp.is_ok() && resp.unwrap().status() == 200
}

pub async fn login(email: &str, password: &str) -> anyhow::Result<()> {
    let path = format!("/login?email={email}&password={password}");
    let resp = get(&path).send().await?;
    if resp.status() != 200 {
        let message = resp.json::<OperResult>().await.unwrap().message;
        return Err(anyhow::anyhow!(
            match message.as_str() {
                "data not found" => "User Info Not Found".to_string(),
                _ => message
            }
        ));
    }
    return Ok(());
}