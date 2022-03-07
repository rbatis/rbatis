use std::fs::File;
use rbatis_core::db::DBPool;
use rbatis_core::Error;
use rbson::{bson, Bson};

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::fs::create_dir_all("target/sqlite/");
    let f = File::create("target/sqlite/test.db");
    drop(f);
    let pool = DBPool::new("sqlite://target/sqlite/test.db").await?;
    let mut conn = pool.acquire().await?;
    //
    conn.exec_sql("CREATE TABLE biz_activity(id int, b boolean);").await;
    conn.exec_sql("insert into biz_activity (id,b) values (1,true)").await;

    let data: (Bson, usize) = conn
        .fetch("SELECT * FROM biz_activity limit 1;")
        .await.unwrap();
    return Ok(());
}