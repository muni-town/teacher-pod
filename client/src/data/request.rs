use reqwasm::http::{Request, Response};
use url::Url;

const SERVER: &'static str = "http://localhost:3000/";

pub async fn get(url: &str) -> anyhow::Result<Response> {
    let resp = Request::get(
        &Url::parse(SERVER)?.join(url)?.to_string()
    )
        .send()
        .await?;
    Ok(resp)
}