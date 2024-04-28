//Angel Lores - CS 410P - Question Server
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
struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>
}
/* Future struct for Answer
struct Answer {
    id: String,
    content: String,
    q_id: String
} 
*/
//Implementing Question's new instance w/ args id, title, content, tags
impl Question {
    fn new(
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

//STORE Struct & Impl
#[derive(Clone)]
struct Store {
    //questions: Vec<Question>, //swapped for HashMap which is better for our purposes
    questions: HashMap<String, Question>,
    //answers: HashMap<String, Answer>
}
impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }
    //Prefill for testing
    fn prefill(&mut self) {
        let q = vec![
            Question::new(
                "8050",
                "Program",
                "How do we implement this?",
                &["cs", "rust", "web_dev"]
            ),
            Question::new(
                "1010",
                "Graduation",
                "When is the grad fair?",
                &["cs", "e"]
            ),
        ];
        q.into_iter().for_each(|q| self.post(q));
    }
    //GET all (Read)
    fn get(&self) -> &HashMap<String, Question> {
        &self.questions
    }
    //POST (Create)
    fn post(&mut self, q: Question) {
        self.questions.insert(q.id.clone(), q);
    }
    //PUT (Update) 
    //DELETE
}
//MAIN
#[tokio::main]
async fn main() {
    let mut s = Store::new();
    s.prefill();
    let s = Arc::new(RwLock::new(s));
    //Mostly taken from Bart's Knock Knock
    let app = Router::new()
        .route("/", post(post_op))
        .route("/", get(get_op))
        //.route("/", put(put_op))
        //.route("/", delete(delete_op))
        .with_state(s); //necessary or error with axum::serve
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("http://{}/", ip);
    axum::serve(listener, app).await.unwrap();
} 
//Get all questions
async fn get_op(State(s): State<Arc<RwLock<Store>>>) -> impl IntoResponse {
    let q: Vec<Question> = s.read().await.get().values().cloned().collect();
    Json(q)
}
//Post a question (probably need to make sure id does not exist already?)
async fn post_op(State(s): State<Arc<RwLock<Store>>>, Json(q): Json<Question>) {
    s.write().await.post(q);
}
//Put (Update) a question
//Delete a question