use super::{
    db_entity::SnippetInfo,
    es_entity::{HitsArray, PostDataResponse, SearchResponse},
};
use reqwest::Client;

async fn create_index(index: &str) {
    let mut url = String::new();
    url.push_str("http://127.0.0.1:9200/");
    url.push_str(index);
    let res = Client::new()
        .put(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    match serde_json::from_str(&res) {
        Ok(data) => data,
        Err(err) => {
            println!("{:?}", err.to_string());
            return;
        }
    };
}

pub async fn post_data(index: &str, snippet_info: SnippetInfo) -> bool {
    create_index(index).await;
    let mut url = String::new();
    url.push_str("http://127.0.0.1:9200/");
    url.push_str(index);
    url.push_str("/_doc");
    let json_body = serde_json::to_string(&snippet_info).unwrap();
    let res = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .body(json_body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res: PostDataResponse = serde_json::from_str(&res).unwrap();
    if res.result == "created" || res.result == "updated" {
        true
    } else {
        false
    }
}

// todo 将pg的数据插入到es中，创建对应的索引，根据描述search出结果
pub async fn search_data(index: &str, desc: String) -> Vec<HitsArray> {
    let mut url = String::new();
    url.push_str("http://127.0.0.1:9200/");
    url.push_str(index);
    url.push_str("/_search");
    let mut search_json = String::new();
    search_json.push_str(r#"{"query": {"match":{"description":""#);
    search_json.push_str(&desc);
    search_json.push_str(r#"""#);
    search_json.push_str(r#"}}}"#);
    let res = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .body(search_json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res: SearchResponse = serde_json::from_str(&res).unwrap();
    res.hits.hits
}
