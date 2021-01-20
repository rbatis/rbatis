#[cfg(test)]
mod test {
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use crate::BizActivity;


    /// ctx_id Used to trace SQL records
    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0"
    )]
    async fn py_ctx_id(rb: &Rbatis, ctx_id: &str) -> Vec<BizActivity> {}

    #[async_std::test]
    pub async fn test_py_ctx_id() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_ctx_id(&rb, "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }


    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_select_page(rb: &Rbatis, page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select_page(&rb, &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }

    ///Commit the transaction
    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_sql_tx(rb: &Rbatis, tx_id: &String, name: &str) -> Vec<BizActivity> {}

    #[async_std::test]
    pub async fn test_py_sql_tx() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        //guard will be Automatically commit or rollback transactions
        let tx_guard=rb.begin_tx_defer(true).await.unwrap();
        let a = py_sql_tx(&rb, &tx_guard.tx_id, "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
