#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;

    #[tokio::test]
    pub async fn test_dyn_table_name() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut w = rb.new_wrapper();
        //replace your dyn table name
        w.formats
            .insert("table_name".to_string(), |s|"biz_activity".to_string());
        //support all of RB.*_wrapper() method
        let r = rb.fetch_list_by_wrapper::<BizActivity>(&w).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }
}
