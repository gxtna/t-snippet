use db::db_entity::SnippetInfo;
use serde::{Deserialize, Serialize};
use tokio;
mod db;
mod server;
mod utils;
// github client_id :cfc1410aa53dc97243dd
// github Client secret :54d59c9d64d3c672dde8bd9a2f410544c6063d70
#[tokio::main]
 async fn main() {
    server::web_server::web_server_route().await;
}



