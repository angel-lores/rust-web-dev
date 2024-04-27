mod question;

use question::Question;

use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

use axum::{
    //extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response, Redirect},
    routing::{get, post, put, delete},
    Json, 
    Router,
};


#[tokio::main]
async fn main() {
    /* Testing Question 
    // Creating a new question
    let id = "1";
    let title = "Sample Question";
    let content = "This is a sample question content.";
    let tags = &["tag1", "tag2", "tag3"];
    let q = Question::new(id, title, content, tags);
    // Printing the question
    println!("Question ID: {}", q.id);
    println!("Title: {}", q.title);
    println!("Content: {}", q.content);
    if let Some(tags) = &q.tags {
        println!("Tags: {:?}", tags);
    } else {
        println!("No tags specified.");
    }
    */
    let app = Router::new()
        .route("/", get(get_questions));
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("{}", ip);
    axum::serve(listener, app).await.unwrap();
}
async fn get_questions() -> impl IntoResponse {
    let tags = &["tag1", "tag2", "tag3"];
    let q = Question::new(
        "782",
        "Example Question",
        "Example Content",
        tags
    );
    Json(q)
}