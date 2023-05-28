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
        snippet_id: String,
        user_id: String,
        title: String,
        tags: String,
        description: String,
        content: String,
    ) -> Self {
        let snippet_id = match snippet_id.len() > 0 {
            true => snippet_id,
            false => nanoid::nano_id(),
        };
        Self {
            snippet_id,
            user_id,
            title,
            tags,
            description,
            content,
            create_time: time::get_local_time(),
            update_time: time::get_local_time(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow,Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub nick_name: String,
    pub avatar_url: String,
    pub account: String,
    pub password: String,
    pub solt: String,
    pub email: String,
    pub description: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}
impl UserInfo {
    pub fn git_login(nick_name: String, avatar_url: String) -> Self {
        Self {
            user_id: nanoid::nano_id(),
            nick_name,
            avatar_url,
            account: "".to_string(),
            password: "".to_string(),
            solt: "".to_string(),
            email: "".to_string(),
            description: "".to_string(),
            create_time: time::get_local_time(),
            update_time: time::get_local_time(),
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct TagInfo {
    pub tag_id: String,
    pub tag_name: String,
}

pub struct OhterLoginInfo {
    pub user_id: String,
    pub login_type: String,
}