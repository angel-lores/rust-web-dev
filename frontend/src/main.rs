//Angel Lores - CS410P - Question Server Frontend
use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

//QUESTION Struct
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Question {
    id: i32,
    title: String,
    content: String,
}

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(Vec::new);

    {
        let questions = questions.clone();
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://127.0.0.1:3000/q/").send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Vec<Question>>().await {
                                Ok(fetched_questions) => {
                                    web_sys::console::log_1(&"Questions fetched successfully".into());
                                    web_sys::console::log_1(&format!("{:?}", fetched_questions).into());
                                    questions.set(fetched_questions);
                                }
                                Err(err) => {
                                    web_sys::console::log_1(&"Failed to parse JSON".into());
                                    web_sys::console::log_1(&format!("{:?}", err).into());
                                }
                            }
                        } else {
                            web_sys::console::log_1(&"Request failed".into());
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&"Failed to fetch questions".into());
                        web_sys::console::log_1(&format!("{:?}", err).into());
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <div>
                { format!("[ID] Title - Content") }
                { for questions.iter().map(|question| html! { <li>{ format!("[{}] {} - {}", question.id, question.title, question.content) }</li> }) }
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}