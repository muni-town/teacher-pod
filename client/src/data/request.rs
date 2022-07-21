use gloo::net::http::Request;
use url::Url;

// const SERVER: &'static str = "http://localhost:3000/";

pub async fn get(url: &str) -> Request {
    let url = &Url::parse(root_path().await)
        .unwrap()
        .join(url)
        .unwrap()
        .to_string();
    Request::get(url)
}

pub async fn root_path() -> &'static str {
    let resp = Request::get("/root.txt").send().await;
    if let Ok(resp) = resp {
        let res = Box::leak(Box::new(resp.text().await.unwrap()));
        if res.is_empty() {
            return "http://146.190.40.243:3000/";
        } else {
            return res;
        }
    }
    "http://146.190.40.243:3000/"
}