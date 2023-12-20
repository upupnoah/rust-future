use std::net::SocketAddr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // initialize logging
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=root dbname=postgres password=root",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();
    let app = Router::new()
        .route("/query_from_db", get(query_from_db))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
        .with_state(pool);

    serve(app, 3089).await;
}

// region:    --- serve
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
// endregion: --- serve

// region:    --- handlers
type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn query_from_db(State(pool): State<ConnectionPool>) -> Result<String, (StatusCode, String)> {
    tracing::debug!("get db coon {:?}", pool);
    let conn = pool
        .get()
        .await
        // .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        .map_err(internal_error)?;

    tracing::debug!("query_from_db: 1");
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;

    tracing::debug!("query_from_db: 2");
    let two: i32 = row.try_get(0).map_err(internal_error)?;
    tracing::debug!("query_from_db: 3");
    tracing::debug!("calc_result {:?}", two);

    Ok(two.to_string())
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
// endregion: --- handlers

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
