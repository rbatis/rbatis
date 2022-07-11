#[cfg(test)]
mod test {
    use crate::init_sqlite;
    use rbatis::core::db::DBPool;

    //示例-Rbatis直接使用驱动
    #[tokio::test]
    pub async fn test_use_driver() {
        init_sqlite().await;

        fast_log::init(fast_log::config::Config::new().console());
        let pool = DBPool::new("sqlite://../target/sqlite.db").await.unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let (r, _) = conn
            .fetch::<serde_json::Value>("select count(1) from biz_activity;")
            .await
            .unwrap();
        println!("done:{:?}", r);
    }
}
