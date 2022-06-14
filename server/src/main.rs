mod api;
mod auth;
mod db;
mod error;
mod models;
mod task;

use api::account::AccountApi;
use db::{get_postgres, init_pg_pool};
use salvo::{
    extra::cors::CorsHandler,
    hyper::header::{ACCEPT, AUTHORIZATION},
    prelude::*,
};
use task::schedule_task;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub trait Routers {
    fn build() -> Vec<Router>;
}

#[fn_handler]
async fn all_pass(res: &mut Response) {
    res.set_status_code(StatusCode::OK);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    dotenv::dotenv().ok();
    let _ = init_pg_pool().await;
    schedule_task(get_postgres().clone());

    let cors_handler = CorsHandler::builder()
        .with_allow_any_origin()
        .with_allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .with_allow_headers(vec![AUTHORIZATION, ACCEPT])
        .build();

    let router = Router::with_hoop(cors_handler)
        .append(AccountApi::build())
        .push(Router::with_path("<*path>").options(all_pass));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
