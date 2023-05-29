use std::collections::HashMap;

use crate::db::{
    db_entity::{SnippetInfo, TagInfo, UserInfo},
    db_server,
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
use crate::utils::nanoid;

pub async fn web_server_route() {
    // 解决跨域
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(index))
        .route("/github_login", get(github_login))
        .route("/get_snippet", get(get_snippet))
        .route("/get_all_snippets", get(get_all_snippets))
        .route("/write_snippet", post(write_snippet))
        .route("/get_all_tags", get(get_all_tags))
        .layer(cors);
    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    return "http://127.0.0.1:8080".to_string();
}
async fn github_login(Query(map): Query<HashMap<String, String>>) -> Json<GitHubUserInfo> {
    let code = map.get("code").unwrap().to_string();
    let mut url = String::new();
    url.push_str("https://github.com/login/oauth/access_token?client_id=cfc1410aa53dc97243dd&client_secret=54d59c9d64d3c672dde8bd9a2f410544c6063d70&code=");
    url.push_str(&code);
    let client = Client::new()
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    let res = match client {
        Ok(client) => match client.text().await {
            Ok(text) => text,
            Err(e) => {
                println!("{}", e);
                e.to_string()
            }
        },
        Err(e) => {
            println!("{}", e);
            e.to_string()
        }
    };
    let access: GithubAccessToken = match serde_json::from_str(&res) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let mut token = String::new();
    token.push_str("Bearer ");
    token.push_str(&access.access_token);
    let user = match Client::new()
        .get("https://api.github.com/user")
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "t-snippet")
        .header("Authorization", token)
        .send()
        .await
    {
        Ok(client) => match client.text().await {
            Ok(text) => text,
            Err(e) => {
                println!("{}", e);
                e.to_string()
            }
        },
        Err(e) => {
            println!("{}", e);
            e.to_string()
        }
    };
    let user_info: GitHubUserInfo = match serde_json::from_str(&user) {
        Ok(text) => text,
        Err(_) => todo!(),
    };
    let info = UserInfo::git_login(user_info.clone().login, user_info.clone().avatar_url);
    let x = db_server::insert_or_update_user(info).await;
    let res = GitHubUserInfo{
        login: user_info.login,
        avatar_url: user_info.avatar_url,
        user_id: Some(x),
    };
    Json(res)
}

async fn write_snippet(Json(snippet_info): Json<WebSnippetInfo>) -> Json<bool> {
    let tags: String = serde_json::to_string(&snippet_info.tags).unwrap();
    println!("{:?}", snippet_info);
    let temp = snippet_info.snippet_id.clone();
    let mut snippet_info = SnippetInfo::new(
        snippet_info.snippet_id,
        snippet_info.user_id,
        snippet_info.title,
        tags,
        snippet_info.desc,
        snippet_info.content,
    );
    
    match temp.len() > 0 {
        true => {
            snippet_info.snippet_id = temp;
            let res = db_server::update_sinppet(snippet_info).await;
            Json(res)
        }
        false => {
            snippet_info.snippet_id = nanoid::nano_id();
            let res = db_server::inset_snippet(snippet_info).await.unwrap();
            Json(res)
        }
    }
}

async fn get_snippet(Query(snippet_id): Query<HashMap<String, String>>) -> Json<SnippetInfo> {
    let info = db_server::get_sinppet(snippet_id.get("snippet_id").unwrap().to_string()).await;
    Json(info)
}

async fn get_all_snippets() -> Json<Vec<SnippetInfo>> {
    let list = db_server::select_snippet_list().await;
    Json(list)
}

async fn get_all_tags() -> Json<Vec<TagInfo>> {
    let tags = db_server::get_all_tags().await;
    Json(tags)
}

#[derive(Debug, Deserialize, Serialize)]
struct WebSnippetInfo {
    snippet_id: String,
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
#[derive(Debug, Deserialize, Serialize, Clone)]
struct GitHubUserInfo {
    login: String,
    avatar_url: String,
    user_id: Option<String>,
}
