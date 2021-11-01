#[cfg(test)]
mod test {
    use crate::BizActivity;
    use chrono::NaiveDateTime;
    use rbatis::core::types::DateTimeNative;
    use rbatis::core::value::DateTimeNow;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::rbatis::Rbatis;

    #[tokio::test]
    async fn plugin_exclude_delete() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        let mut plugin = RbatisLogicDeletePlugin::new("delete_flag");
        rb.set_logic_plugin(plugin);
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let id = "12312".to_string();
        //logic delete sql:   "update biz_activity set delete_flag = 1 where id = ?"
        rb.remove_by_column::<BizActivity, _>("id", &id).await;
        //delete sql          "delete from biz_activity where id = ?"
        rb.remove_by_wrapper::<BizActivity>(rb.new_wrapper().set_dml("delete").eq("id", &id)).await;

        //fix data
        rb.save(
            &BizActivity {
                id: Some("12312".to_string()),
                name: Some("12312".to_string()),
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: Some("1".to_string()),
                status: Some(1),
                remark: None,
                create_time: Some(DateTimeNative::now()),
                version: Some(0),
                delete_flag: Some(1),
            },
            &[]
        )
            .await;
    }
}
