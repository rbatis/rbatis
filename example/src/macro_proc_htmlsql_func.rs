#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use rbson::Bson;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::{BizActivity, init_sqlite};

    pub trait IsFine{
        fn is_test(&self)->bool;
    }
    impl IsFine for rbson::Bson{
        fn is_test(&self) -> bool {
            match self{
                Bson::String(v) => {
                    v.eq("test")
                }
                _ => {
                    false
                }
            }
        }
    }


    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[html_sql("example/example.html")]
    async fn custom_func(rb: &mut RbatisExecutor<'_, '_>, page_req: &PageRequest, name: &str, dt: &rbatis::DateTimeNative) -> Page<BizActivity> { impled!() }

    #[tokio::test]
    pub async fn test_custom_func() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = custom_func(&mut rb.as_executor(), &PageRequest::new(1, 10), "test", &rbatis::DateTimeNative::now())
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
