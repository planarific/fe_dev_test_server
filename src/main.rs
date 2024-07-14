mod data;
mod model;

use axum::routing::get;
use std::net::SocketAddr;
use std::thread;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::data::DATA;
use crate::model::Model;

async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    })
    .join()
    .unwrap()
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

pub async fn get_health() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

pub async fn get_models() -> axum::Json<Vec<Model>> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        data.values().map(|m| m.clone()).collect::<Vec<_>>()
    })
    .join()
    .unwrap()
    .into()
}

pub async fn get_models_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<axum::Json<Model>, axum::http::StatusCode> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(model) => Ok(axum::Json(model.clone())),
            None => Err(axum::http::StatusCode::NOT_FOUND),
        }
    })
    .join()
    .unwrap()
    .into()
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // print_data().await;

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/health", get(get_health))
        .route("/models", get(get_models))
        .route("/models/:id", get(get_models_id))
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        );

    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
