use crate::*;

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

//implement tag format?

