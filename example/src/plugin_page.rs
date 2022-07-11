#[cfg(test)]
mod test {
    use crate::{init_sqlite, BizActivity};
    use once_cell::sync::Lazy;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rbson::Bson;

    pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

    #[tokio::test]
    pub async fn test_sql_page() {
        init_sqlite().await;
        fast_log::init(fast_log::config::Config::new().console());
        let rb = Rbatis::new();
        rb.link("sqlite://../target/sqlite.db").await.unwrap();
        let data: Page<BizActivity> = rb
            .fetch_page_by_wrapper(
                rb.new_wrapper().eq("delete_flag", 0),
                &PageRequest::new(1, 20),
            )
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    /// RB is the name of the RBatis object
    /// Pysql is the middle string data
    #[py_sql(
        RB,
        "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}"
    )]
    async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_macro_py_select_page() {
        init_sqlite().await;
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        RB.link("sqlite://../target/sqlite.db").await.unwrap();
        let a = py_select_page(&PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }

    #[py_sql(RB, "select * from biz_activity group by id")]
    async fn group_by(page_req: &PageRequest) -> Page<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_group_by_page() {
        init_sqlite().await;
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        RB.link("sqlite://../target/sqlite.db").await.unwrap();
        let a = group_by(&PageRequest::new(1, 10)).await.unwrap();
        println!("{:?}", a);
    }
}
