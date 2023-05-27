use crate::utils::{nanoid, time};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct SnippetInfo {
    pub snippet_id: String,
    pub user_id: String,
    pub title: String,
    pub tags: String,
    pub description: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl SnippetInfo {
    pub fn new(
        user_id: String,
        title: String,
        tags: String,
        description: String,
        content: String,
    ) -> Self {
        Self {
            snippet_id: nanoid::nano_id(),
            user_id,
            title,
            tags,
            description,
            content,
            create_time: time::get_local_time(),
            update_time: time::get_local_time(),
        }
    }

    pub fn snippet_id(&self) -> String {
        self.snippet_id.to_string()
    }
    pub fn user_id(&self) -> String {
        self.user_id.to_string()
    }
    pub fn tags(&self) -> String {
        self.tags.to_string()
    }
    pub fn description(&self) -> String {
        self.description.to_string()
    }
    pub fn content(&self) -> String {
        self.content.to_string()
    }
}

pub struct UserInfo {
    pub user_id: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub description: String,
    pub token: String,
    pub token_expired: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

pub struct TagInfo {
    pub tag_id: i32,
    pub name: String,
}

pub struct OhterLoginInfo {
    pub user_id: String,
    pub login_type: String,
}
