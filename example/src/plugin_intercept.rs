#[cfg(test)]
mod test {
    use rbson::Bson;
    use crate::{BizActivity, init_sqlite};
    use rbatis::core::Error;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::intercept::{
        BlockAttackDeleteInterceptor, BlockAttackUpdateInterceptor, SqlIntercept,
    };
    use rbatis::rbatis::Rbatis;
    use serde_json::Value;

    #[tokio::test]
    pub async fn test_block_attack() {
        fast_log::init(fast_log::config::Config::new().console());
        let mut rb = init_sqlite().await;
        rb.add_sql_intercept(BlockAttackDeleteInterceptor {});
        rb.add_sql_intercept(BlockAttackUpdateInterceptor {});
        let r = rb.exec("delete from biz_activitys", vec![]).await;
        if r.is_err() {
            println!("block success:{}", r.err().unwrap());
        }
    }

    #[derive(Debug)]
    pub struct MyIntercept {}

    impl SqlIntercept for MyIntercept {
        fn do_intercept(
            &self,
            rb: &Rbatis,
            sql: &mut String,
            args: &mut Vec<Bson>,
            is_prepared_sql: bool,
        ) -> Result<(), rbatis::core::Error> {
            println!(">>>>>> hello this is my inercept!>>>>>>");
            println!(">>>>>> my inercept:->  sql: {}", sql);
            println!(">>>>>> my inercept:-> args: {:?}", args);
            return Result::Ok(());
        }
    }

    #[tokio::test]
    pub async fn test_intercept() {
        fast_log::init(fast_log::config::Config::new().console());
        let mut rb = init_sqlite().await;
        rb.add_sql_intercept(MyIntercept {});
        let w = rb.new_wrapper().eq("id", "1");
        let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper(w).await;
    }
}
