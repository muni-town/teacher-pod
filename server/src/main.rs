mod api;
mod error;
mod model;

use api::*;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_database_sessions::{AxumSessionConfig, AxumSessionLayer, AxumSessionStore};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, Any};
use std::{net::SocketAddr, time::Duration};

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
    let session = AxumSessionStore::new(
        Some(pool.clone().into()),
        AxumSessionConfig::default().with_table_name("sessions"),
    );
    session.migrate().await.unwrap();

    let app = Router::new()
        .route("/login", get(account::login))
        .route("/register", post(account::register))
        .route("/self", get(account::self_info))
        .route("/users/:id", get(users::get_user))
        .layer(Extension(pool))
        .layer(AxumSessionLayer::new(session))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}