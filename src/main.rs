
use tokio;
mod server;
mod db;
mod utils;

#[tokio::main]
async fn main() {
    server::web_server::web_server_route().await;
}
