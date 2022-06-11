mod api;
mod auth;
mod error;
mod model;

use api::*;

use axum::{
    http::header::{ACCEPT, AUTHORIZATION},
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

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

    let task_pool = pool.clone();
    tokio::task::spawn(async move {
        let pool = task_pool;
        loop {
            let sql = format!(
                "delete from auth where expire <= {};",
                chrono::Local::now().timestamp()
            );
            let r = sqlx::query(&sql).execute(&pool).await;
            if let Err(e) = r {
                log::warn!("clean auth data failed: {}", e.to_string());
            }
            // two day task
            tokio::time::sleep(std::time::Duration::from_secs(60 * 60 * 24 * 5)).await;
        }
    });

    let app = Router::new()
        .route("/login", get(account::login))
        .route("/register", post(account::register))
        .route("/self", get(account::self_info))

        .route("/users/:id", get(users::get_user))

        .route("/topics/", get(topics::popular_topics))
        .route("/topics/recommend", get(topics::topic_recommend))
        .route("/topics/:id", get(topics::get_topic))
        
        .layer(Extension(pool))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(vec![AUTHORIZATION, ACCEPT]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
