use std::collections::HashMap;

use crate::{
    db::{db::SnippetInfo, db_server},
    utils::time,
};
use axum::{
    self,
    extract::Json,
    extract::Query,
    routing::{get, post},
    Router, Server,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use tower_http::cors::CorsLayer;

pub async fn web_server_route() {
    // 解决跨域
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(index))
        .route("/github_login", get(github_login))
        .route("/get_snippet", get(get_snippet))
        .route("/get_all_snippets", get(get_all_snippets))
        .route("/write_snippet", post(write_snippet))
        .layer(cors);
    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    return "http://127.0.0.1:8080".to_string();
}
async fn github_login(Query(code): Query<GithubInfo>) -> Json<GitHubUserInfo> {
    let mut url = String::new();
    url.push_str("https://github.com/login/oauth/access_token?client_id=cfc1410aa53dc97243dd&client_secret=54d59c9d64d3c672dde8bd9a2f410544c6063d70&code=");
    url.push_str(&code.code);
    let res = Client::new()
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", res);
    let access: GithubAccessToken = serde_json::from_str(&res).unwrap();
    let mut token = String::new();
    token.push_str("Bearer ");
    token.push_str(&access.access_token);
    let user = Client::new()
        .get("https://api.github.com/user")
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "t-snippet")
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let user_info: GitHubUserInfo = serde_json::from_str(&user).unwrap();
    Json(user_info)
}

async fn write_snippet(Json(snippet_info): Json<WebSnippetInfo>) -> Json<bool> {
    let tags: String = serde_json::to_string(&snippet_info.tags).unwrap();
    let snippet_info = SnippetInfo::new(
        snippet_info.user_id,
        snippet_info.title,
        tags,
        snippet_info.desc,
        snippet_info.content,
    );
    let res = db_server::inset_snippet(snippet_info).await;
    Json(res)
}

async fn get_snippet(Query(snippet_id): Query<HashMap<String,String>>) -> Json<SnippetInfo> {
    let info = db_server::get_sinppet(snippet_id.get("snippet_id").unwrap().to_string()).await;
    Json(info)
}

async fn get_all_snippets() -> Json<Vec<SnippetInfo>> {
    let list = db_server::select_snippet_list().await;
    Json(list)
}

#[derive(Debug, Deserialize, Serialize)]
struct WebSnippetInfo {
    user_id: String,
    title: String,
    tags: Vec<String>,
    desc: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GithubAccessToken {
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct GithubInfo {
    code: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct GitHubUserInfo {
    login: String,
    avatar_url: String,
}
