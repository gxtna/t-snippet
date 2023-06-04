use super::{
    db_entity::SnippetInfo,
    es_entity::{DeleteResponse, HitsArray, PostDataResponse, SearchResponse},
};
use crate::utils::constant::APPCONFIG;
use reqwest::Client;

async fn create_index() {
    let es = &APPCONFIG.elasticsearch;
    let mut url = String::new();
    url.push_str(&es.url);
    url.push_str(&es.index);
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
            return;
        }
    };
}

pub async fn post_data(snippet_info: SnippetInfo) -> bool {
    create_index().await;
    let es = &APPCONFIG.elasticsearch;
    let mut url = String::new();
    url.push_str(&es.url);
    url.push_str(&es.index);
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
pub async fn search_data(desc: String) -> Vec<HitsArray> {
    let es = &APPCONFIG.elasticsearch;
    let mut url = String::new();
    url.push_str(&es.url);
    url.push_str(&es.index);
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

pub async fn delete_data(desc: String) -> bool {
    let es = &APPCONFIG.elasticsearch;
    let mut url = String::new();
    url.push_str(&es.url);
    url.push_str(&es.index);
    url.push_str("/_delete_by_query ");
    let mut search_json = String::new();
    search_json.push_str(r#"{"query": {"match":{"snippet_id":""#);
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
    let res: DeleteResponse = serde_json::from_str(&res).unwrap();
    res.deleted > 0
}
