
use sqlx::{postgres::PgConnection, PgPool, Postgres};

async fn create_connection_pool() -> PgConnection {
    // TODO 如果有数据库其他配置需要修改
    let pool = PgPool::connect("postgres://root:123456@localhost:5432/database")
        .await
        .unwrap();
    pool.acquire().await.unwrap().detach()
}