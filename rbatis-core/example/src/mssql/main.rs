use rbatis_core::db::DBPool;

#[tokio::main]
async fn main(){
    let pool = DBPool::new("mssql://SA:TestPass!123456@localhost:1433/test").await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
}