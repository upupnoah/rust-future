use std::net::SocketAddr;

use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // initialize logging

    tokio::join!(
        serve(serve_hello(), 3089),
        serve(serve_dir(), 3090),
        serve(serve_dir_with_asserts_fallback(), 3091),
        serve(serve_query(), 3092),
    );
}

// region:    --- Serve
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    // Middleware
    let app = app.layer(TraceLayer::new_for_http());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// endregion: --- Serve

// region:    --- handler
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
// endregion: --- handler

// region:    --- Routes
fn serve_hello() -> Router {
    Router::new().route("/", get(handler))
}

fn serve_dir() -> Router {
    // 将./example/assets -> /assets
    // 当前目录是项目根目录
    Router::new().nest_service("/assets", ServeDir::new("examples/assets"))
}

fn serve_dir_with_asserts_fallback() -> Router {
    let serve_dir = ServeDir::new("examples/assets")
        .not_found_service(ServeFile::new("examples/assets/index.html"));
    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
}

fn serve_query() -> Router {
    Router::new().route("/query", get(query))
}

// endregion: --- Routes

// region:    --- Query
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>, // aa is optional
}

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("query params {:?}", params);
    Html("<h1>Test query</h1>")
}
// endregion: --- Query
