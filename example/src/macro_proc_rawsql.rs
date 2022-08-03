#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::sql::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::{init_sqlite, BizActivity};

    /// doc you can see https://rbatis.github.io/rbatis.io/#/en/
    #[sql("select * from biz_activity where delete_flag = ?")]
    async fn sql_fn(rb: &Rbatis, delete_flag: &i32) -> Vec<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_sql_fn() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = sql_fn(&rb, &1).await.unwrap();
        println!("{:?}", a);
    }
}
