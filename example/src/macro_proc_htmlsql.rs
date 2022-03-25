#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::{BizActivity, init_sqlite};

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[html_sql("example/example.html")]
    async fn select_by_condition(rb: &mut RbatisExecutor<'_, '_>, page_req: &PageRequest, name: &str, dt: &rbatis::DateTimeNative) -> Page<BizActivity> { impled!() }

    #[tokio::test]
    pub async fn test_select_by_condition() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = select_by_condition(&mut rb.as_executor(), &PageRequest::new(1, 10), "test", &rbatis::DateTimeNative::now())
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
