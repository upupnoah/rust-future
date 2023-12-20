use std::net::SocketAddr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // initialize logging

    // region:    --- Init db state
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=root dbname=todolist password=root",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    // endregion: --- Init db state

    let routes_all = Router::new()
        .route("/todos", get(list_todo))
        .route("/todo/new", post(create_todo))
        .route("/todo/update", post(update_todo))
        .route("/todo/delete/:id", post(delete_todo))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    serve(routes_all, 3089).await;
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

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: String,
    description: String,
    completed: bool,
}

// region:    --- handlers

#[derive(Debug, Deserialize)]
struct CreateTodo {
    description: String,
}
async fn create_todo(
    State(pool): State<ConnectionPool>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let todo = Todo {
        id: Uuid::new_v4().simple().to_string(),
        description: input.description,
        completed: false,
    };

    let conn = pool.get().await.map_err(internal_error)?;

    let _ret = conn
        .execute(
            "insert into todo (id, description, completed) values ($1, $2, $3) returning id",
            &[&todo.id, &todo.description, &todo.completed],
        )
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    id: String,
    description: Option<String>,
    completed: Option<bool>,
}
async fn update_todo(
    State(pool): State<ConnectionPool>,
    Json(utodo): Json<UpdateTodo>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let update_todo = UpdateTodo {
        id: utodo.id,
        description: utodo.description,
        completed: utodo.completed,
    };

    tracing::debug!(
        "id, desc, completed {:?} {:?} {:?}",
        update_todo.id,
        update_todo.description,
        update_todo.completed
    );

    let _ret = conn
        .execute(
            "update todo set description=$2, completed=$3 where id=$1",
            &[
                &update_todo.id,
                &update_todo.description,
                &update_todo.completed.unwrap_or(false),
            ],
        )
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(update_todo.id)))
}

async fn delete_todo(
    Path(id): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let _ret = conn
        .execute("delete from todo where id=$1", &[&id])
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::OK, Json(id)))
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
async fn list_todo(
    State(pool): State<ConnectionPool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let Query(pagination) = pagination.unwrap_or_default();
    let offset = pagination.offset.unwrap_or(0);
    let limit = pagination.limit.unwrap_or(100);

    let rows = conn
        .query(
            "select id, description, completed from todo offset $1 limit $2",
            &[&offset, &limit],
        )
        .await
        .map_err(internal_error)?;

    let mut todos: Vec<Todo> = Vec::new();
    for row in rows {
        let id = row.get(0);
        let description = row.get(1);
        let completed = row.get(2);
        let todo = Todo {
            id,
            description,
            completed,
        };
        todos.push(todo);
    }
    Ok(Json(todos))
}
// endregion: --- handlers

// region:    --- Internal error
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// endregion: --- Internal error
