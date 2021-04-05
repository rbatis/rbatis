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

    #[tokio::test]
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

    #[tokio::test]
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

    #[tokio::test]
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
    fn load_file_str(file_name: &str, method: &str) -> String {
        //load all string
        let mut f = File::open(file_name).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);

        //find method
        let methods: Vec<&str> = s.split("--fn:").collect();
        for x in methods {
            if x.starts_with(method) {
                let data = x.trim_start_matches(method).to_string();
                return data;
            }
        }
        panic!("not find method:'{}' in file:'{}'", method, file_name)
    }

    ///load file py_sql(Each read file changes every time)
    #[py_sql(rb, load_file_str("py_sql.sql", "py_select_file"))]
    async fn py_select_file(rb: &Rbatis, page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    lazy_static! {
        pub static ref PY_SQL_FILE_STR: String = load_file_str("py_sql.sql", "py_select_file");
    }

    ///load file py_sql(only load file once)
    #[py_sql(rb, PY_SQL_FILE_STR)]
    async fn py_select_file_static(
        rb: &Rbatis,
        page_req: &PageRequest,
        name: &str,
    ) -> Page<BizActivity> {
    }

    /// test load py_sql from file
    #[tokio::test]
    pub async fn test_py_select_file() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut result = py_select_file(&rb, &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", result);

        result = py_select_file_static(&rb, &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", result);
    }
}
