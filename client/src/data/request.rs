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
        let res = resp.text().await;
        if res.is_err() {
            return "https://teacherpod-server.mrxzx.info/";
        } else {
            let res = res.unwrap();
            if res.is_empty() || resp.status() != 200 {
                return "https://teacherpod-server.mrxzx.info/";
            }
            return Box::leak(Box::new(res));
        }
    }
    "https://teacherpod-server.mrxzx.info/"
}
