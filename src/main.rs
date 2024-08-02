mod data;
mod images;
mod model;

use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;

use dotenv;
use std::net::SocketAddr;
use std::ops::Deref;
use std::thread;
use uuid::Uuid;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use http::Method;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::data::DATA;
use crate::images::obfuscate;
use crate::model::{Model, ModelListEntry};

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

pub async fn get_models() -> axum::Json<Vec<ModelListEntry>> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let data = obfuscate(data.deref());
        data.values()
            .map(|m| ModelListEntry {
                id: m.id,
                thumbnail: m.thumbnail.clone(),
                description: format!("{} {}", m.address1, m.postal_code),
            })
            .collect::<Vec<_>>()
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
        let data = obfuscate(data.deref());
        match data.get(&id) {
            Some(model) => Ok(axum::Json(model.clone())),
            None => Err(axum::http::StatusCode::NOT_FOUND),
        }
    })
    .join()
    .unwrap()
    .into()
}

pub async fn get_thumbs_id(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let v: Vec<&str> = id.split(".png").collect();
    let uuid = Uuid::parse_str(v[0]).unwrap();
    match images::unhash_thumbnail(&uuid) {
        Some(str) => Redirect::to(&str).into_response(),
        None => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
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

    let cors = CorsLayer::new()
        // .allow_methods([Method::GET, Method::POST, Method::HEAD])
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // print_data().await;

    dotenv::dotenv().ok();

    let token = dotenv::var("TOKEN").unwrap();

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/v1/models", get(get_models))
        .route("/v1/models/:id", get(get_models_id))
        .route("/v1/thumbs/:id", get(get_thumbs_id))
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        )
        .route_layer(ValidateRequestHeaderLayer::bearer(&token))
        .route("/v1/health", get(get_health))
        .layer(cors);

    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let host = [0, 0, 0, 0];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
