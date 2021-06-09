#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    lazy_static! {
       pub static ref RB: Rbatis = Rbatis::new();
    }

    #[tokio::test]
    pub async fn test_sql_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let wraper = rb.new_wrapper().eq("delete_flag", 0);
        let data: Page<BizActivity> = rb
            .fetch_page_by_wrapper(&wraper, &PageRequest::new(1, 20))
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }


    #[py_sql(RB, "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_macro_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select_page(&PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }

    #[py_sql(RB, "select * from biz_activity group by id")]
    async fn group_by(page_req: &PageRequest) -> Page<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_group_by_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = group_by(&PageRequest::new(1, 10)).await.unwrap();
        println!("{:?}", a);
    }
}
