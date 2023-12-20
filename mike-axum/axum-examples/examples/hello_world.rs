use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3089));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
