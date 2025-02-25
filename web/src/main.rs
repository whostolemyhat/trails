use axum::{
    Router,
    http::{HeaderMap, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde_derive::Deserialize;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use extractors::AppJson;

mod err;
mod extractors;

// use path not line
// env
// serve static
// cors

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

async fn generate(AppJson(payload): AppJson<Payload>) -> impl IntoResponse {
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

struct ApplicationSettings {
    port: u16,
    host: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = ApplicationSettings {
        port: env::var("PORT")
            .unwrap_or(String::from("5678"))
            .parse::<u16>()
            .expect("Couldn't parse port"),
        host: env::var("HOST").unwrap_or("localhost".into()),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/api/generate", post(generate))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", settings.host, settings.port))
        .await
        .expect("Failed to create listener");
    tracing::info!(
        "Listening on  {}",
        listener.local_addr().expect("No listener address")
    );
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
