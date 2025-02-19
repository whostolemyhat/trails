use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_derive::Deserialize;

// logging
// error
// parse json
// remove unwrap
// use path not line
// env
// serve static

async fn home() -> &'static str {
    "Hello world!"
}

#[derive(Debug, Deserialize)]
struct Payload {
    seed: String,
    canvas_size: usize,
    min_leaf_size: usize,
    density: u8,
}

async fn generate(Json(payload): Json<Payload>) -> String {
    dbg!(&payload);
    // check payload
    let image = trails::create(
        &payload.seed,
        payload.canvas_size,
        payload.min_leaf_size,
        payload.density,
    );
    image
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/api/generate", post(generate));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
