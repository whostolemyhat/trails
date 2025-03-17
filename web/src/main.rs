use axum::{
    Router,
    http::{
        HeaderMap, HeaderValue, Method, StatusCode,
        header::{self, CONTENT_TYPE},
    },
    response::{Html, IntoResponse},
    routing::{get, post},
};
use minijinja::{Environment, context};
use serde_derive::Deserialize;
use std::{
    env,
    fs::read_to_string,
    time::{SystemTime, UNIX_EPOCH},
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use extractors::AppJson;

mod err;
mod extractors;

async fn home() -> impl IntoResponse {
    let template_path = env::var("TEMPLATE_PATH").unwrap_or(String::from("./frontend/dist"));
    let mut app_env = Environment::new();
    let index_template = format!("{}/index.html", template_path);
    let template_content = read_to_string(index_template).expect("Couldn't read template");

    app_env
        .add_template("home", &template_content)
        .expect("Failed to load template");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Somehow time has failed")
        .as_millis();
    let template = app_env.get_template("home").expect("Couldn't get template");

    let density = 2;
    let canvas_size = 45;
    let leaf_size = 3;
    let image = trails::create(&now.to_string(), canvas_size, leaf_size, density);

    Html(
        template
            .render(context! { seed => now, image => image, density => 2, size => 45, leaf => 3 })
            .expect("Failed to render"),
    )
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

async fn not_found() -> impl IntoResponse {
    let template_path = env::var("TEMPLATE_PATH").unwrap_or(String::from("./frontend/dist"));
    let template = format!("{}/404.html", template_path);
    let template_content = read_to_string(template).expect("Couldn't read template");

    (StatusCode::NOT_FOUND, Html(template_content))
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

    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let settings = ApplicationSettings {
        port: env::var("PORT")
            .unwrap_or(String::from("5678"))
            .parse::<u16>()
            .expect("Couldn't parse port"),
        host: env::var("HOST").unwrap_or("localhost".into()),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        // TODO
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .route("/", get(home))
        .route("/api/generate", post(generate))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .layer(compression_layer)
        .layer(cors);

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
