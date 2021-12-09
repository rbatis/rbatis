#[cfg(test)]
mod test {
    use rbatis::core::db::DBPool;

    //示例-Rbatis直接使用驱动
    #[tokio::test]
    pub async fn test_use_driver() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let (r, _) = conn
            .fetch::<serde_json::Value>("select count(1) from biz_activity;")
            .await
            .unwrap();
        println!("done:{:?}", r);
    }
}
