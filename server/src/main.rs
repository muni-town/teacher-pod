mod api;
mod auth;
mod error;
mod model;

use api::*;

use axum::{
    routing::{get, post},
    Extension, Router, http::header::{AUTHORIZATION, ACCEPT},
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

    let app = Router::new()
        .route("/login", get(account::login))
        .route("/register", post(account::register))
        .route("/self", get(account::self_info))
        .route("/users/:id", get(users::get_user))
        .route("/contents/:id", get(contents::get_content))
        .layer(Extension(pool))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(vec![AUTHORIZATION, ACCEPT]));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
