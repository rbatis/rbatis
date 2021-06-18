#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::core::Error;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::intercept::{
        BlockAttackDeleteInterceptor, BlockAttackUpdateInterceptor, SqlIntercept,
    };
    use rbatis::rbatis::Rbatis;
    use serde_json::Value;
    use rbatis::executor::Executor;

    #[tokio::test]
    pub async fn test_block_attack() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        rb.add_sql_intercept(BlockAttackDeleteInterceptor {});
        rb.add_sql_intercept(BlockAttackUpdateInterceptor {});
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let r = rb.exec("delete from biz_activitys", &vec![]).await;
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
            args: &mut Vec<Value>,
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
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        rb.add_sql_intercept(MyIntercept {});
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let w = rb.new_wrapper().eq("id", "1");
        let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper(&w).await;
    }
}
