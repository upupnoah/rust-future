use std::net::SocketAddr;

use axum::{response::Html, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3088));
    println!("Listening on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1> Hello, World!!!! </h1>")
}
