#[cfg(test)]
mod test {
    use crate::{init_sqlite, BizActivity};
    use rbdc::db::ExecResult;
    use rbatis::executor::{ExecutorMut, RBatisTxExecutor, RbatisExecutor, RbatisRef};
    use rbatis::rbatis::Rbatis;
    use std::cell::Cell;

    //示例-Rbatis使用事务
    #[tokio::test]
    pub async fn test_tx_commit() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb: Rbatis = init_sqlite().await;
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
    async fn py_select_data(
        rb: &mut RbatisExecutor<'_, '_>,
    ) -> Result<Vec<BizActivity>, rbatis::core::Error> {
        impled!()
    }

    //示例-Rbatis使用宏事务
    #[tokio::test]
    pub async fn test_tx_py() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb: Rbatis = init_sqlite().await;
        let mut tx = rb.acquire_begin().await.unwrap();
        let v = py_select_data(&mut tx.as_executor()).await.unwrap();
        println!("{:?}", v);
        tx.commit().await.unwrap();
    }

    //示例-Rbatis使用事务,类似golang defer，守卫如果被回收则 释放事务
    #[tokio::test]
    pub async fn test_tx_commit_defer() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb: Rbatis = init_sqlite().await;
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
