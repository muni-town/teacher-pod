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
    listener::TcpListener,
    Router, Server,
};
use task::schedule_task;

pub trait Routers {
    fn build() -> Vec<Router>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let _ = init_pg_pool().await;
    schedule_task(get_postgres().clone());

    let cors_handler = CorsHandler::builder()
        .with_allow_any_origin()
        .with_allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .with_allow_headers(vec![AUTHORIZATION, ACCEPT])
        .build();

    let router = Router::with_hoop(cors_handler).append(AccountApi::build());

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
