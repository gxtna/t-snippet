use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub database: DatabaseConfig,
    pub elasticsearch: ElasticSearchConfig,
    pub github: GitHubConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: String,
    pub url: String,
    pub port: i32,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ElasticSearchConfig {
    pub url: String,
    pub index: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token_url: String,
    pub user_info_url: String,
}
