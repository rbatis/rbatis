#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;
    use rbatis::executor::{Executor, RbatisRef, RBatisTxExecutor, ExecutorMut, RbatisExecutor};
    use rbatis::core::db::DBExecResult;
    use std::cell::Cell;
    use async_std::sync::Mutex;

    //示例-Rbatis使用事务
    #[tokio::test]
    pub async fn test_tx_commit() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let mut tx = rb.acquire_begin().await.unwrap();

        let v = tx
            .exec("update biz_activity set name = '6' where id = 1;", vec![])
            .await
            .unwrap();

        println!("{:?}", v);
        tx.commit().await.unwrap();

        let v: serde_json::Value = rb
            .fetch("select * from biz_activity where id = 1;", vec![])
            .await
            .unwrap();
        println!("result:{}", v.to_string());
    }

    #[py_sql(rb, "select * from biz_activity")]
    async fn py_select_data(rb: &mut RbatisExecutor<'_, '_>) -> Result<Vec<BizActivity>, rbatis::core::Error> { todo!() }

    //示例-Rbatis使用宏事务
    #[tokio::test]
    pub async fn test_tx_py() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut tx = rb.acquire_begin().await.unwrap();
        let v = py_select_data(&mut tx.as_executor()).await.unwrap();
        println!("{:?}", v);
        tx.commit().await.unwrap();
    }

    //示例-Rbatis使用事务,类似golang defer，守卫如果被回收则 释放事务
    #[tokio::test]
    pub async fn test_tx_commit_defer() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        let rb: Rbatis = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        forget_commit(&rb).await.unwrap();
    }

    pub async fn forget_commit(rb: &Rbatis) -> rbatis::core::Result<()> {
        // tx will be commit.when func end
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx| async move {
            if !tx.is_done() {
                tx.rollback().await;
                println!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        let v = tx
            .exec("update biz_activity set name = '6' where id = 1;", vec![])
            .await;
        //tx.commit().await;  //if commit, print 'don't need rollback!' ,if not,print 'tx rollback success!'
        return Ok(());
    }
}
