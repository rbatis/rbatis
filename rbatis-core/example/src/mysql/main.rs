use rbatis_core::db::DBPool;
use rbatis_core::Error;
use rbson::{bson,Bson};

#[tokio::main]
async fn main() -> Result<(), Error> {
    //Automatic judgment of database type or  postgres://postgres:123456@localhost:5432/postgres
    let pool = DBPool::new("mysql://root:123456@localhost:3306/test").await?;
    let mut conn = pool.acquire().await?;
    let data: (Bson, usize) = conn
        .fetch("SELECT * FROM biz_activity limit 1;")
        .await.unwrap();
    return Ok(());
}
