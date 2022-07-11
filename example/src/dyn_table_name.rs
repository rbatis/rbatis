#[cfg(test)]
mod test {
    use crate::{init_sqlite, BizActivity};
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;

    #[tokio::test]
    pub async fn test_dyn_table_name() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb = init_sqlite().await;

        let mut w = rb.new_wrapper();
        //replace your dyn table name
        //w.formats.insert("table_name".to_string(),  "biz_{}".to_string());//also support {} is replace value!
        w.formats
            .insert("table_name".to_string(), "biz_activity".to_string());
        //support all of RB.*_wrapper() method
        let r = rb.fetch_list_by_wrapper::<BizActivity>(w).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }
}
