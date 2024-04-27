use serde::{Serialize, Deserialize};
use std::{net::SocketAddr, collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response, Redirect},
    routing::{get, post, put, delete},
    Json, 
    Router
};

//QUESTION Struct & Impl
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>
}
//Implementing Question's new instance w/ args id, title, content, tags
impl Question {
    pub fn new(
        id: &str,
        title: &str,
        content: &str,
        tags: &[&str],
    ) -> Self {
        let id = id.into();
        let title = title.into();
        let content = content.into();
        let tags: Option<Vec<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            title,
            content,
            tags
        }
    }
}

//STORE
#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<String, Question>>>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init()))
        }
    }

    fn init() -> HashMap<String, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_questions));
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("http://{}/", ip);
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