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
        serve(serve_hello(), 3088),
        serve(serve_dir(), 3089),
        serve(serve_dir_with_asserts_fallfack(), 3090),
        serve(serve_query(), 3091),
    );
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", addr);
    axum::serve(
        listener,
        app.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .await
    .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

fn serve_hello() -> Router {
    let app = Router::new().route("/", get(handler));
    app
}

fn serve_dir() -> Router {
    // serve the file in "assets" directory under '/assets'
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}

fn serve_dir_with_asserts_fallfack() -> Router {
    // `serveDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/does-not-exist.jpg` will return `index.html`
    // rather than a 404
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
}

fn serve_query() -> Router {
    Router::new().route("/query", get(query))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>, // optional query parameter
}
// http://localhost:3091/query?foo=123&bar=noah&&aa=123

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}
