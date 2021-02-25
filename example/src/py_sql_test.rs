#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use std::fs::File;
    use std::io::Read;

    /// ctx_id Used to trace SQL records
    #[py_sql(rb, "select * from biz_activity where delete_flag = 0")]
    async fn py_ctx_id(rb: &Rbatis, ctx_id: &str) -> Vec<BizActivity> {}

    #[async_std::test]
    pub async fn test_py_ctx_id() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_ctx_id(&rb, "test").await.unwrap();
        println!("{:?}", a);
    }

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
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
        let tx_guard = rb.begin_tx_defer(true).await.unwrap();
        let a = py_sql_tx(&rb, &tx_guard.tx_id, "test").await.unwrap();
        println!("{:?}", a);
    }


    ///load from file
    fn load_file_str(file_name:&str)->String{
        let mut f =File::open(file_name).unwrap();
        let mut s=String::new();
        f.read_to_string(&mut s);
        return s;
    }

    ///load file py_sql
    #[py_sql(rb, load_file_str("py_sql.sql"))]
    async fn py_select_file(rb: &Rbatis, page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_py_select_file() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select_file(&rb, &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
