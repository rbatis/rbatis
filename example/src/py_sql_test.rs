#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    #[py_sql(rb, "select * from biz_activity where delete_flag = 0","mysql")]
    async fn py_ctx_id(rb: &Rbatis) -> Vec<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_py_ctx_id() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_ctx_id(&rb).await.unwrap();
        println!("{:?}", a);
    }

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}","mysql"
    )]
    async fn py_select_page(rb: &mut RbatisExecutor<'_>, page_req: &PageRequest, name: &str) -> Page<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select_page(&mut (&rb).into(), &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }

    ///Commit the transaction
    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}","mysql")]
    async fn py_sql_tx(rb: &Rbatis, tx_id: &String, name: &str) -> Vec<BizActivity> { todo!() }


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
}
