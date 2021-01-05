#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;
    use rbatis::plugin::page::{Page, PageRequest, RbatisPagePlugin, RbatisPackPagePlugin, RbatisReplacePagePlugin};
    use rbatis::crud::CRUD;

    lazy_static! { static ref RB:Rbatis=Rbatis::new();}

    #[async_std::test]
    pub async fn test_sql_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let wraper = rb.new_wrapper()
            .eq("delete_flag", 0).check().unwrap();
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper, &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }


    #[async_std::test]
    pub async fn test_py_sql_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let py = r#"
    SELECT * FROM biz_activity
    WHERE delete_flag = #{delete_flag}
    if name != null:
      AND name like #{name+'%'}"#;
        let data: Page<BizActivity> = rb.py_fetch_page("", py, &serde_json::json!({   "delete_flag": 1 }), &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    #[py_sql(RB, "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    async fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select_page(&PageRequest::new(1, 10), "test").await.unwrap();
        println!("{:?}", a);
    }


    #[async_std::test]
    pub async fn test_choose_page_plugin() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        //set plugin
        let mut plugin = RbatisPagePlugin::new();
        plugin.plugins.push(Box::new(RbatisPackPagePlugin {}));
        plugin.plugins.push(Box::new(RbatisReplacePagePlugin {}));
        rb.page_plugin = Box::new(plugin);
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let wraper = rb.new_wrapper()
            .eq("delete_flag", 0)
            .check()
            .unwrap();
        //choose pack page plugin
        let plugin_name = std::any::type_name::<RbatisPackPagePlugin>().to_string();
        println!("plugin_name:{}", &plugin_name);
        let mut req = PageRequest::new_plugin(plugin_name, 1, 10, 0);
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper, &req).await.unwrap();
        //choose replace page plugin
        req.plugin = std::any::type_name::<RbatisReplacePagePlugin>().to_string();
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper, &req).await.unwrap();
    }
}