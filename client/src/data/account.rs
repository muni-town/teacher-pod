use super::request::get;

pub async fn is_login() -> bool {
    let resp = get("/self").send().await;
    resp.is_ok() && resp.unwrap().status() == 200
}

pub async fn login(email: &str, password: &str) -> anyhow::Result<()> {
    let path = format!("/login?email={email}&password={password}");
    let _ = get(&path).send().await?;
    return Ok(());
}