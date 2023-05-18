use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct SnippetInfo {
    snippet_id: String,
    user_id: String,
    tags: String,
    description: String,
    content: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl SnippetInfo{

    pub fn snippet_id(&self) -> String{
        self.snippet_id.to_string()
    }
    pub fn user_id(&self) -> String{
        self.user_id.to_string()
    }
    pub fn tags(&self) -> String{
        self.tags.to_string()
    }
    pub fn description(&self) -> String{
        self.description.to_string()
    }
    pub fn content(&self) -> String{
        self.content.to_string()
    }
}