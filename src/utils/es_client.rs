use anyhow::Ok;
use reqwest::Client;
use url::Url;

#[derive(Debug, Clone)]
pub struct ESClient {
    url: Url,
    client: Client,
}
// TODO 待实现
impl ESClient {
    pub fn new(url: &str) -> ESClient {
        let url = Url::parse(url).unwrap();

        ESClient {
            url,
            client: Client::new(),
        }
    }

}
