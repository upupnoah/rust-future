use std::net::SocketAddr;

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // initialize logging

    let app = Router::new()
        .route("/greet/:name/:age", get(greet)) // Get http://localhost:3089/greet/Noah/23
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    serve(app, 3089).await;
}

// region:    --- serve
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
// endregion: --- serve

// region:    --- handlers
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
    age: u8,
}
async fn greet(Path((name, age)): Path<(String, u8)>) -> impl IntoResponse {
    let template = HelloTemplate { name, age };
    Html(template.render().unwrap())
}

async fn handler_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "nothing to see here!")
}
// endregion: --- handlers
