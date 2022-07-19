mod api;
mod auth;
mod db;
mod error;
mod models;
mod task;

mod listennotes;

use api::{account::AccountApi, podcast::PodcstApi, search::SearchApi};
use db::{get_postgres, init_pg_pool};
use salvo::{
    extra::cors::CorsHandler,
    hyper::header::{ACCEPT, AUTHORIZATION},
    prelude::*,
};
use task::schedule_task;

pub trait Routers {
    fn build() -> Vec<Router>;
}

#[fn_handler]
async fn hello_world(res: &mut Response) {
    res.render("Hello World!");
}

#[fn_handler]
async fn all_pass(res: &mut Response) {
    res.set_status_code(StatusCode::OK);
}

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("sqlx", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    dotenv::dotenv().ok();
    let _ = init_pg_pool().await;
    schedule_task(get_postgres().clone());

    let mock_info = std::env::var("MOCK_API").unwrap();
    if mock_info.to_lowercase() == "true" {
        log::warn!("currently using mock api data.");
    }

    let cors_handler = CorsHandler::builder()
        .with_allow_any_origin()
        .with_allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .with_allow_headers(vec![AUTHORIZATION, ACCEPT])
        .build();

    let router = Router::with_hoop(cors_handler)
        .push(Router::with_path("/").get(hello_world))
        .append(AccountApi::build())
        .append(PodcstApi::build())
        .append(SearchApi::build())
        .push(Router::with_path("<*path>").options(all_pass));

    let server_addr = std::env::var("SERVER_ADDR").unwrap();
    Server::new(TcpListener::bind(&server_addr))
        .serve(router)
        .await;
}
