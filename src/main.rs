mod db;
mod api;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;


#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracaing
    tracing_subscriber::fmt::init();

    // db connect
    let  pool = db::establish_connection().await;

    let app = Router::new()
        .route("/", get(root))
        .route("/api/wx_counter/login", post(api::user::login))
        .route("/api/wx_counter/counters", get(api::counter::list))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:30000").await.unwrap();
    info!("server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str {
    "hello axum"
}