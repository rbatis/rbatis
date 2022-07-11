#[cfg(test)]
mod test {
    use crate::{init_sqlite, BizActivity};

    use rbatis::core::value::DateTimeNow;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::rbatis::Rbatis;
    use rbatis::DateTimeNative;

    #[tokio::test]
    async fn plugin_exclude_delete() {
        fast_log::init(fast_log::config::Config::new().console());
        let mut rb = init_sqlite().await;
        let mut plugin = RbatisLogicDeletePlugin::new("delete_flag");
        rb.set_logic_plugin(plugin);

        let id = "12312".to_string();
        //logic delete sql:   "update biz_activity set delete_flag = 1 where id = ?"
        rb.remove_by_column::<BizActivity, _>("id", &id).await;
        //delete sql          "delete from biz_activity where id = ?"
        rb.remove_by_wrapper::<BizActivity>(rb.new_wrapper().set_dml("delete").eq("id", &id))
            .await;

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
            &[],
        )
        .await;
    }
}
