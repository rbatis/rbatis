#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    #[sql("select * from biz_activity where delete_flag = ?")]
    async fn sql_fn(rb: &Rbatis, delete_flag: &i32) -> Vec<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_sql_fn() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = sql_fn(&rb, &1).await.unwrap();
        println!("{:?}", a);
    }
}
