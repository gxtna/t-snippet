use axum::{self, Router, Server,routing::get};
use crate::db::db_server;


pub async fn web_server_route() {
    let app = Router::new().route("/", get(index));
    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    let res = db_server::select_snippet_list().await;
    return "http://127.0.0.1:8080".to_string();
}
