use serde::{Deserialize, Serialize};
use super::db_entity::SnippetInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateIndexResponse {
    pub acknowledged: bool,
    pub shards_acknowledged: bool,
    pub index: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDataResponse {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: i32,
    pub result: String,
    #[serde(rename = "_shards")]
    pub shards: ShardsBody,
    #[serde(rename = "_seq_no")]
    pub seq_no: i32,
    #[serde(rename = "_primary_term")]
    pub primary_term: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShardsBody {
    pub total: Option<i32>,
    pub successful: Option<i32>,
    pub skipped: Option<i32>,
    pub failed: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub took: i32,
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: ShardsBody,
    pub hits: HitsBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HitsBody {
    pub total: TotalBody,
    pub max_score: f64,
    pub hits: Vec<HitsArray>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalBody {
    pub value: i32,
    pub relation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HitsArray {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: f64,
    #[serde(rename = "_source")]
    pub source: SnippetInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRequest {
    pub query: MatchBody,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MatchBody {
    #[serde(rename = "match")]
    pub match_name: DescBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescBody {
    pub description: String,
}
