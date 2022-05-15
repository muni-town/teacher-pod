use gloo::net::http::Request;
use url::Url;

const SERVER: &'static str = "http://localhost:3000/";

pub fn get(url: &str) -> Request {
    let url = &Url::parse(SERVER).unwrap().join(url).unwrap().to_string();
    Request::get(url)
}
