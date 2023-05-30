use std::collections::HashMap;

//use rs_es::Client;
use crate::utils::es_client::{self, ESClient};
use reqwest::Client;
use rs_es::operations::index;
use serde::{Deserialize, Serialize};

use super::db_entity::SnippetInfo;

pub async fn es_client() {
    /* let mut client = Client::init("http://172.18.0.1:9200").unwrap();
    //let z = client.refresh().send().unwrap();
    client.search_query().
    let x: rs_es::operations::get::GetResult<Source> = client.get("test", "01").with_doc_type("_doc").send().unwrap();
    //println!("{:?}", x.id); */
    let res = Client::new()
        .get("http://172.18.0.1:9200/test/_doc/01")
        .send()
        .await
        .unwrap();
    let x = res.text().await.unwrap();
    println!("{}", x);
}
pub async fn create_index(index: &str) -> bool {
    let mut url = String::new();
    url.push_str("http://172.18.0.1:9200/");
    url.push_str(index);
    let res = Client::new()
        .put(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res: CreateIndexResponse = serde_json::from_str(&res).unwrap();
    res.index == index
}

pub async fn post_data(index: &str, snippet_info: SnippetInfo) {
    let mut url = String::new();
    url.push_str("http://172.18.0.1:9200/");
    url.push_str(index);
    url.push_str("/_doc");
    let json_body = serde_json::to_string(&snippet_info).unwrap();
    let res = Client::new()
        .post(url)
        .body(json_body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", res)
}

// todo 将pg的数据插入到es中，创建对应的索引，根据描述search出结果
pub async fn search(desc: String) {
    let mut url = String::new();
    url.push_str("http://172.18.0.1:9200/");
    url.push_str("test2");
    url.push_str("/_search");
    url.push_str("?q=desc:");
    url.push_str(&desc);

    let res = Client::new()
        .post(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", res);
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateIndexResponse {
    acknowledged: bool,
    shards_acknowledged: bool,
    index: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SearchBody {
    query: MatchObject,
}
#[derive(Debug, Deserialize, Serialize)]
struct MatchObject {
    #[serde(rename = "match")]
    match_name: DescObject,
}
#[derive(Debug, Deserialize, Serialize)]
struct DescObject {
    name: String,
}
