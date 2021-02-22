#[cfg(test)]
mod test {
    use rbatis::rbatis::Rbatis;
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use crate::BizActivity;
    use rbatis::crud::CRUD;

    #[async_std::test]
    async fn plugin_exclude(){
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        let mut plugin=RbatisLogicDeletePlugin::new("delete_flag");
        plugin.excludes.push("exclude:".to_string());
        plugin.excludes.push("tx:exclude:".to_string());
        rb.set_logic_plugin(Some(plugin));
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let id="12312".to_string();
        //logic delete sql:   "UPDATE biz_activity SET delete_flag = 1 WHERE id = ?"
        rb.remove_by_id::<BizActivity>("", &id).await;
        //delete sql          "DELETE FROM biz_activity WHERE id = ?"
        rb.remove_by_id::<BizActivity>("exclude:", &id).await;
    }
}