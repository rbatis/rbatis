#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    /// doc you can see https://rbatis.github.io/rbatis.io/#/en/
    #[sql("select * from biz_activity where delete_flag = ?")]
    async fn sql_fn(rb: &Rbatis, delete_flag: &i32) -> Vec<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_sql_fn() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = sql_fn(&rb, &1).await.unwrap();
        println!("{:?}", a);
    }

    #[py_sql("select a1.name as name,a2.create_time as create_time
                      from test.biz_activity a1,biz_activity a2
                      where a1.id=a2.id
                      and a1.name=#{name}")]
    async fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<BizActivity>> { todo!() }

    #[tokio::test]
    pub async fn test_join() {
        fast_log::init_log("requests.log", log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let results = join_select(&rb, "test").await.unwrap();
        println!("data: {:?}", results);
    }
}
