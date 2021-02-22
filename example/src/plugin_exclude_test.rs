#[cfg(test)]
mod test {
    use rbatis::rbatis::Rbatis;
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use crate::BizActivity;
    use rbatis::crud::CRUD;
    use chrono::NaiveDateTime;
    use rbatis::core::value::DateTimeNow;
    #[async_std::test]
    async fn plugin_exclude_delete(){
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        let mut plugin=RbatisLogicDeletePlugin::new("delete_flag");
        plugin.excludes.push("disable_del:".to_string());
        plugin.excludes.push("tx:disable_del:".to_string());
        rb.set_logic_plugin(Some(plugin));
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let id="12312".to_string();
        //logic delete sql:   "UPDATE biz_activity SET delete_flag = 1 WHERE id = ?"
        rb.remove_by_id::<BizActivity>("", &id).await;
        //delete sql          "DELETE FROM biz_activity WHERE id = ?"
        rb.remove_by_id::<BizActivity>("disable_del:", &id).await;


        //fix data
        rb.save("", &BizActivity {
            id: Some("12312".to_string()),
            name: Some("12312".to_string()),
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: Some("1".to_string()),
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(0),
            delete_flag: Some(1),
        }).await;
    }


    #[async_std::test]
    async fn plugin_exclude_select(){
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        let mut plugin=RbatisLogicDeletePlugin::new("delete_flag");
        plugin.excludes.push("disable_del:".to_string());
        plugin.excludes.push("tx:disable_del:".to_string());
        rb.set_logic_plugin(Some(plugin));
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let id="12312".to_string();
        //logic delete sql:   "SELECT * FROM biz_activity  WHERE delete_flag = 0 AND id = ? "
        rb.fetch_by_id::<BizActivity>("", &id).await;
        //delete sql          "SELECT * FROM biz_activity  WHERE id = ? "
        rb.fetch_by_id::<BizActivity>("disable_del:", &id).await;

    }
}