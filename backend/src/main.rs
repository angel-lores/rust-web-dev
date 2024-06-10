//Angel Lores - CS 410P - Question Server Backend
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
use http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

//QUESTION Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    id: Option<i32>,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

//ANSWER Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: Option<i32>,
    content: String,
    q_id: i32
}

//QUESTION WITH ANSWERS Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct QuestionWithAnswers {
    question: Question,
    answers: Vec<Answer>,
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
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://127.0.0.1:5000"))
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/q/", post(post_q_op).get(get_all_op))
        .route("/a/", post(post_a_op))
        .route("/qa/:id", get(get_op).put(put_op).delete(delete_op))
        .layer(cors)
        .with_state(db_pool);
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("http://{}/q/", ip);
    axum::serve(listener, app).await.unwrap();
}

//HANDLERS using sqlx for CRUD operations to and from the database
async fn post_q_op(State(db_pool): State<PgPool>, Json(new_question): Json<Question>) -> Result<Json<Question>, StatusCode> {
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
async fn post_a_op(State(db_pool): State<PgPool>, Json(new_answer): Json<Answer>) -> Result<Json<Answer>, StatusCode> {
    let answer = sqlx::query!(
        r#"INSERT INTO answer (q_id, content) VALUES ($1, $2) RETURNING id"#,
        new_answer.q_id,
        new_answer.content,
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        println!("Database Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut new_answer = new_answer.clone();
    new_answer.id = Some(answer.id);
    Ok(Json(new_answer))
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
async fn get_op(Path(id): Path<i32>, State(db_pool): State<PgPool>) -> Result<Json<QuestionWithAnswers>, StatusCode> {
    let question = sqlx::query_as!(Question, "SELECT * FROM question WHERE id = $1", id)
        .fetch_optional(&db_pool)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match question {
        Some(question) => {
            let answers = sqlx::query_as!(Answer, "SELECT * FROM answer WHERE q_id = $1", id)
                .fetch_all(&db_pool)
                .await
                .map_err(|e| {
                    println!("Database Error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            let question_with_answers = QuestionWithAnswers {
                question,
                answers,
            };

            Ok(Json(question_with_answers))
        }
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
        Some(question) => Ok(Json(question)), 
        None => Err(StatusCode::NOT_FOUND),
    }
}
async fn delete_op(Path(id): Path<i32>, State(db_pool): State<PgPool>) -> Result<StatusCode, StatusCode> {
    let mut transaction = db_pool.begin().await.map_err(|e| {
        println!("Failed to start transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    //Delete answers associated to the question
    sqlx::query!("DELETE FROM answer WHERE q_id = $1", id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    //Delete question
    sqlx::query!("DELETE FROM question WHERE id = $1", id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            println!("Database Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    //Commit transaction if there are no errors or rollback
    transaction.commit().await.map_err(|e| {
        println!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}