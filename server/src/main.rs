mod auth;
mod model;
mod task;

use std::time::Duration;
use salvo::{Router, Server, listener::TcpListener};
use sqlx::postgres::PgPoolOptions;
use task::schedule_task;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");
    schedule_task(pool.clone());

    let router = Router::new();

    Server::new(TcpListener::bind("127.0.0.1:3000")).serve(router).await;

}
