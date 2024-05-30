//Angel Lores - CS 410P - Question Server
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post}, //, put, delete}, it acts like I am not using these??
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use sqlx::{postgres::PgPoolOptions, PgPool};

//QUESTION Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    id: Option<i32>,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

//MAIN
#[tokio::main]
async fn main() {
    //Accessing database through an "environment variable"
    dotenvy::dotenv().expect("can't access .env file");
    let db_url = std::env::var("DATABASE_URL").expect("db url not found");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("can't connect to db");

    let app = Router::new()
        .route("/", post(post_op).get(get_all_op))
        .route("/:id", get(get_op).put(put_op).delete(delete_op))
        .with_state(db_pool);
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("http://{}/", ip);
    axum::serve(listener, app).await.unwrap();
}

//HANDLERS using sqlx for CRUD operations to and from the database
async fn post_op(State(db_pool): State<PgPool>, Json(new_question): Json<Question>) -> Result<Json<Question>, StatusCode> {
    let question = sqlx::query!(
        r#"INSERT INTO question (title, content, tags) VALUES ($1, $2, $3) RETURNING id"#,
        new_question.title,
        new_question.content,
        new_question.tags.as_deref(),
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        println!("Database Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut new_question = new_question.clone();
    new_question.id = Some(question.id);
    Ok(Json(new_question))
}
async fn get_all_op(State(db_pool): State<PgPool>) -> Result<Json<Vec<Question>>, StatusCode> {
    let question = sqlx::query_as!(Question, "SELECT * FROM question")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(question))
}
async fn get_op(Path(id): Path<i32>, State(db_pool): State<PgPool>) -> Result<Json<Question>, StatusCode> {
    let question = sqlx::query_as!(Question, "SELECT * FROM question WHERE id = $1", id)
        .fetch_optional(&db_pool)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match question {
        Some(question) => Ok(Json(question)), // Use the unwrapped Question
        None => Err(StatusCode::NOT_FOUND),
    }
}
async fn put_op(Path(id): Path<i32>, State(db_pool): State<PgPool>, Json(update_data): Json<Question>) -> Result<Json<Question>, StatusCode> {
    let question = sqlx::query_as!(Question, r#"UPDATE question SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING *"#,
        update_data.title,
        update_data.content,
        update_data.tags.as_deref(),
        id,
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(|e| {
        println!("Database Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match question {
        Some(question) => Ok(Json(question)), // Use the unwrapped Question
        None => Err(StatusCode::NOT_FOUND),
    }
}
async fn delete_op(Path(id): Path<i32>, State(db_pool): State<PgPool>) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DELETE FROM question WHERE id = $1", id)
        .execute(&db_pool)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}