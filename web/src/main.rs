use axum::{
    Json, Router,
    http::{HeaderMap, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde_derive::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// logging
// error
// remove unwrap
// use path not line
// env
// serve static

async fn home() -> &'static str {
    "Hello world!"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    seed: String,
    canvas_size: usize,
    min_leaf_size: usize,
    density: u8,
}

async fn generate(Json(payload): Json<Payload>) -> impl IntoResponse {
    // check payload
    let image = trails::create(
        &payload.seed,
        payload.canvas_size,
        payload.min_leaf_size,
        payload.density,
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/svg+xml".parse().expect("Failed to add svg header"),
    );

    (headers, image)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(home))
        .route("/api/generate", post(generate))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await.unwrap();
    tracing::info!(
        "Listening on  {}",
        listener.local_addr().expect("No listener address")
    );
    axum::serve(listener, app).await.unwrap();
}
