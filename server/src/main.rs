mod auth;
mod models;
mod task;
mod api;
mod db;
mod error;

use api::account::AccountApi;
use db::{get_postgres, init_pg_pool};
use salvo::{Router, Server, listener::TcpListener};
use task::schedule_task;

pub trait Routers {
    fn build() -> Vec<Router>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let _ = init_pg_pool().await;
    schedule_task(get_postgres().clone());

    let router = Router::new().append(AccountApi::build());

    Server::new(TcpListener::bind("127.0.0.1:3000")).serve(router).await;
}
