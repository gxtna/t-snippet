use crate::{
    db::{db::SnippetInfo, db_server},
    utils::time,
};
use axum::{self, extract::Query, routing::get, Json, Router, Server};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use tower_http::cors::{Any, CorsLayer};

pub async fn web_server_route() {
    // 解决跨域
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(index))
        .route("/github_login", get(github_login))
        .route("/callback", get(github_callback))
        .layer(cors);
    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    let time = time::get_local_time();
    let sni = SnippetInfo {
        snippet_id: "1".to_string(),
        user_id: "3".to_string(),
        tags: "3".to_string(),
        description: "3".to_string(),
        content: "3".to_string(),
        create_time: time,
        update_time: time,
    };
    let res = db_server::update_sinppet(sni).await;
    return "http://127.0.0.1:8080".to_string();
}
async fn github_login(Query(access_token): Query<GithubAccessToken>)-> Json<GitHubUserInfo> {

    let mut token = String::new();
    token.push_str("Bearer ");
    token.push_str(&access_token.access_token);
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
async fn github_callback(Query(code): Query<GithubInfo>) ->String {
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
    let access: GithubAccessToken = serde_json::from_str(&res).unwrap();
    access.access_token
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
