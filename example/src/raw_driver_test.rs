#[cfg(test)]
mod test {
    use rbatis::core::db::DBPool;

    //示例-Rbatis直接使用驱动
    #[async_std::test]
    pub async fn test_use_driver() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let pool = DBPool::new("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let (r, _) = conn
            .fetch::<serde_json::Value>("SELECT count(1) FROM biz_activity;")
            .await
            .unwrap();
        println!("done:{:?}", r);
    }
}
