//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PATCH /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cargo run 
//! ```
//! 
use std::clone;
use std::process::id;
use std::{
    sync::{Arc, RwLock},
    collections::HashMap,
};

use axum::extract::Path;
use axum::response::IntoResponse;
use uuid::Uuid;
use axum::{
    extract::{Query, State},
    routing::{get, post, patch, delete},
    http:: StatusCode,
    Json, Router,
};
use serde::{Serialize, Deserialize};



#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/todos", get(todo_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>
}

// get the list of all the todos
async fn todo_index(
    pagination: Option<Query<Pagination>>,
    State(db): State<Db>
) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

// Create a new todo and insert it into map 
async fn todos_create(
    State(db) : State<Db>,
    Json(input): Json<CreateTodo>
) -> impl IntoResponse {

    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

// Update the todos using the id
async fn todos_update(
    Path(id): Path<Uuid>,
    State(db) : State<Db>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }
    
    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}


async fn todos_delete(
    Path(id) : Path<Uuid>,
    State(db): State<Db>,
) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    }
    else {
        StatusCode::NOT_FOUND
    }
}

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}
type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;