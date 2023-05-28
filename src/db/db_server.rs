use super::db_entity::{SnippetInfo, TagInfo, UserInfo};
use crate::utils::time;
use sqlx::{postgres::PgConnection, PgPool, Postgres};
use anyhow::Result;

async fn create_connection_pool() -> PgConnection {
    // TODO 如果有数据库其他配置需要修改
    let pool = PgPool::connect("postgres://root:123456@localhost:5432/database")
        .await
        .unwrap();
    pool.acquire().await.unwrap().detach()
}

pub async fn inset_snippet(snippet_info: SnippetInfo) -> Result<bool> {
    let time = time::get_local_time();
    let mut conn = create_connection_pool().await;
    let res = sqlx::query(
        "insert into snippet_info (snippet_id,user_id,title,tags,description,content,create_time,update_time) values($1,$2,$3,$4,$5,$6,$7,$8)",
    )
    .bind(snippet_info.snippet_id)
    .bind(snippet_info.user_id)
    .bind(snippet_info.title)
    .bind(snippet_info.tags)
    .bind(snippet_info.description)
    .bind(snippet_info.content)
    .bind(time)
    .bind(time)
    .execute(&mut conn)
    .await?;
    Ok(res.rows_affected() == 1)
}

pub async fn select_snippet_list() -> Vec<SnippetInfo> {
    let mut conn = create_connection_pool().await;
    let res = sqlx::query_as::<Postgres, SnippetInfo>("select * from snippet_info")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    res
}

pub async fn get_sinppet(snippet_id: String) -> SnippetInfo {
    let mut conn = create_connection_pool().await;
    let res =
        sqlx::query_as::<Postgres, SnippetInfo>("select * from snippet_info where snippet_id = $1")
            .bind(snippet_id)
            .fetch_one(&mut conn)
            .await
            .unwrap();
    res
}

pub async fn update_sinppet(snippet_info: SnippetInfo) -> bool {
    let time = time::get_local_time();
    let mut conn = create_connection_pool().await;
    let res = sqlx::query(
        "update snippet_info set tags = $1 ,description=$2,content=$3 ,update_time = $4 ,title= $5 where snippet_id = $6",
    )
    .bind(snippet_info.tags)
    .bind(snippet_info.description)
    .bind(snippet_info.content)
    .bind(time)
    .bind(snippet_info.title)
    .bind(snippet_info.snippet_id)
    .execute(&mut conn)
    .await
    .unwrap();
    res.rows_affected() == 1
}
pub async fn delete_snippet(snippet_id: String) -> bool {
    let mut conn = create_connection_pool().await;
    let res = sqlx::query("delete from snippet_info where snippet_id = $1")
        .bind(snippet_id)
        .execute(&mut conn)
        .await
        .unwrap();
    res.rows_affected() == 1
}

pub async fn get_all_tags() -> Vec<TagInfo> {
    let mut conn = create_connection_pool().await;
    let res = sqlx::query_as::<Postgres, TagInfo>("select * from tag_info")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    res
}

pub async fn insert_user_info(user_info: UserInfo) -> String {
    let mut conn = create_connection_pool().await;
    let time = time::get_local_time();
    let info = user_info.clone();
    let res = sqlx::query("insert into user_info (user_id, nick_name, avatar_url ,password, solt, email, account, description, create_time, update_time) 
    values ($1, $2, $3, $4, $5, $6,$7, $8, $9, $10)")
    .bind(user_info.user_id)
    .bind(user_info.nick_name)
    .bind(user_info.avatar_url)
    .bind(user_info.account)
    .bind(user_info.password)
    .bind(user_info.solt)
    .bind(user_info.email)
    .bind(user_info.description)
    .bind(time).bind(time)
    .execute(&mut conn).await.unwrap();
    match res.rows_affected() == 1 {
        true => info.user_id,
        false => "".to_string(),
    }
}
