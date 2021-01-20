#[cfg(test)]
mod test {
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    #[py_sql(
    rb,
    "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_select_page(rb: &Rbatis, page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select_page() {
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
}
