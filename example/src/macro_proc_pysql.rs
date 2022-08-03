#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::sql::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::{init_sqlite, BizActivity};

    #[py_sql("select * from biz_activity where delete_flag = 0")]
    async fn py_ctx_id(rb: &Rbatis) -> Vec<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_py_ctx_id() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = py_ctx_id(&rb).await.unwrap();
        println!("{:?}", a);
    }

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[py_sql(
        "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_select_page(
        mut rb: RbatisExecutor<'_>,
        page_req: &PageRequest,
        name: &str,
    ) -> Page<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_py_select_page() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = py_select_page(rb.as_executor(), &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
