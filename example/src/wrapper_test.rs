#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::crud::CRUD;
    use rbatis::rbatis::Rbatis;

    #[async_std::test]
    pub async fn test_use_driver_wrapper() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let name = "test";
        let w = rb
            .new_wrapper()
            .push_sql("select count(1) from biz_activity where ")
            .r#in("delete_flag", &[0, 1])
            .and()
            .ne("delete_flag", -1)
            .do_if(!name.is_empty(), |w| w.and().like("name", name));
        let r: Vec<BizActivity> = rb.fetch_list_by_wrapper("", &w).await.unwrap();
        println!("done:{:?}", r);
    }
}
