use std::net::SocketAddr;

use crate::{log::log_request, model::ModelController};

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // let routes_hello = Router::new()
    //     .route(
    //         "/hello",
    //         // get(|| async { Html("Hello <strong>World!!!</strong>") }),
    //         get(handler_hello),
    //     )
    //     .route("/hello2/:name", get(handler_hello2));

    // Initialize ModelController
    let mc = ModelController::new().await.unwrap();

    // 这个中间件仅作用于 routes_apis
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // merge routes
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(
            // ServiceBuilder 符合直觉, 自上而下, 依次添加中间件, layer是自下而上
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(middleware::from_fn_with_state(
                    mc.clone(),
                    web::mw_auth::mw_ctx_resolver,
                ))
                .layer(middleware::map_response(main_response_mapper)),
        )
        .fallback_service(routes_static());

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3089));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server
    Ok(())
}

/// consume Response, return different Response(modify, or not)
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:12} - main-response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();

    // convert the error into a client error.
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                    "error" : {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                    }
                }
            );
            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error body.
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    // println!("    ->> server log line - {uuid} - Error: {service_error:?}");
    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    // http://localhost:3089/index.html -> ./index.html
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routes Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// 模式匹配, 从 Query 中提取参数
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

fn routes_hello() -> Router {
    Router::new()
        .route(
            "/hello",
            // get(|| async { Html("Hello <strong>World!!!</strong>") }),
            get(handler_hello),
        )
        .route("/hello2/:name", get(handler_hello2))
}
// endregion: --- Routes Hello
