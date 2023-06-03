use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebSnippetInfo {
    pub  snippet_id: String,
    pub  user_id: String,
    pub  title: String,
    pub tags: Vec<String>,
    pub desc: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubAccessToken {
    pub  access_token: String,
    pub  scope: String,
    pub token_type: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubUserInfo {
    pub  login: String,
    pub  avatar_url: String,
    pub user_id: Option<String>,
}