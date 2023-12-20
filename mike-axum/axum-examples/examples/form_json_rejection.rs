use std::net::SocketAddr;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // initialize logging

    let router_all = Router::new()
        .route("/hello", get(serve_hello))
        .route("/form", post(accept_form).get(show_form))
        .route("/json", post(accept_json))
        .route("/users", post(create_user))
        .route("/resjson", post(res_json))
        .route("/resjson2", post(res_json2))
        .fallback(handler_404);

    serve(router_all, 3089).await;
}

// region:    --- serve
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    // Middleware
    let app = app.layer(TraceLayer::new_for_http());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
// endregion: --- serve

// region:    --- handlers
async fn serve_hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);
    Html("<h3>Form posted</h3>")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
// endregion: --- handlers

// region:    --- Form
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Input {
    name: String,
    email: String,
}
// endregion: --- Form

// region:    --- Json
async fn accept_json(Json(input): Json<Input>) -> Html<&'static str> {
    tracing::debug!("json params {:?}", input);
    Html("<h3>Json posted</h3>")
}
// endregion: --- Json

// region:    --- Rejection Example
use serde_json::{json, Value};
async fn create_user(payload: Result<Json<Value>, JsonRejection>) {
    match payload {
        Ok(_) => {}
        Err(JsonRejection::MissingJsonContentType(_)) => {}
        Err(_) => {}
    }
}
// endregion: --- Rejection Example

// region:    --- Custom Extractor

// Implement FromRequest or FromRequestParts for your own type (cannot both)

// endregion: --- Custom Extractor

// region:    --- The return value of a handler

// as long as the return value implements the IntoResponse trait.

// async fn handler1() -> impl IntoResponse {
//     Html("<h1>Hello, world!</h1>")
// }

// async fn handler2() -> impl IntoResponse {
//     "<h1>Hello, world!"
// }

// async fn handler3() -> impl IntoResponse {
//     Json(json!({
//         "message": "Hello, world!",
//     }))
// }

// async fn handler4() -> impl IntoResponse {
//     Redirect::to("/")
// }

async fn res_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    Json(json!({
        "result": "ok",
        "number": 1,
    }))
}

#[derive(Serialize, Debug, Clone)]
struct Output {
    name: String,
    age: u32,
}

async fn res_json2(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    let output = Output {
        name: "noah".to_string(),
        age: 23,
    };
    // let json = json!({
    //     "name": "noah",
    //     "age": "23",
    // });
    // assert_eq!(serde_json::to_value(output.clone()).unwrap(), json);
    Json(serde_json::to_value(output).unwrap())
}

// endregion: --- The return value of a handler
