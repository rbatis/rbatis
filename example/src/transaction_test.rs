#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;
    use rbatis::executor::{Executor, RbatisRef, RBatisTxExecutor};

    //示例-Rbatis使用事务
    #[tokio::test]
    pub async fn test_tx_commit() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let tx = rb.acquire_begin().await.unwrap();
        let v: serde_json::Value = tx
            .fetch("select count(1) from biz_activity;",&vec![])
            .await
            .unwrap();
        println!("{}", v.clone());
        tx.commit().await.unwrap();
    }

    #[py_sql(rb, "select * from biz_activity")]
    async fn py_select_data(rb: &mut RBatisTxExecutor<'_>) -> Result<Vec<BizActivity>, rbatis::core::Error> { todo!()}

    //示例-Rbatis使用宏事务
    #[tokio::test]
    pub async fn test_tx_py() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut tx = rb.acquire_begin().await.unwrap();
        let v = py_select_data(&mut tx).await.unwrap();
        println!("{:?}", v);
        tx.commit().await.unwrap();
    }

    //示例-Rbatis使用事务,类似golang defer，守卫如果被回收则 释放事务
    #[tokio::test]
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
        let tx = rb.acquire_begin().await?.defer(|tx|{
            println!("tx is drop!");
            async_std::task::block_on(async{ tx.rollback().await; });
        });
        let v: serde_json::Value = tx
            .fetch( "select count(1) from biz_activity;",&vec![])
            .await?;
        return Ok(v);
    }
}
