#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;

    //示例-Rbatis使用事务
    #[async_std::test]
    pub async fn test_tx_commit() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let tx_id = rb.begin_tx().await.unwrap();
        let v: serde_json::Value = rb
            .fetch(&tx_id, "select count(1) from biz_activity;")
            .await
            .unwrap();
        println!("{}", v.clone());
        rb.commit(&tx_id).await.unwrap();
    }

    //tx_id: is context_id
    #[py_sql(rb, "select * from biz_activity")]
    async fn py_select_data(
        rb: &Rbatis,
        tx_id: &str,
    ) -> Result<Vec<BizActivity>, rbatis::core::Error> {
    }

    //示例-Rbatis使用宏事务
    #[async_std::test]
    pub async fn test_tx_py() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let tx_id = rb.begin_tx().await.unwrap();
        let v = py_select_data(&rb, &tx_id).await.unwrap();
        println!("{:?}", v);
        rb.commit(&tx_id).await.unwrap();
    }

    //示例-Rbatis使用事务,类似golang defer，守卫如果被回收则 释放事务
    #[async_std::test]
    pub async fn test_tx_commit_defer() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        forget_commit(&rb).await.unwrap();
    }

    pub async fn forget_commit(rb: &Rbatis) -> rbatis::core::Result<serde_json::Value> {
        // tx will be commit.when func end
        let guard = rb.begin_tx_defer(true).await?;
        let v: serde_json::Value = rb
            .fetch(&guard.tx_id, "select count(1) from biz_activity;")
            .await?;
        return Ok(v);
    }
}
