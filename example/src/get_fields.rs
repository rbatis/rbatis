#[cfg(test)]
mod test {
    use crate::{init_sqlite, BizActivity};
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;

    /// This example shows a table collection  to an id array
    #[tokio::test]
    pub async fn test_fetch_by_ids() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb = init_sqlite().await;
        let biz_activitys = rb.fetch_list::<BizActivity>().await.unwrap();
        let ids = rbatis::make_table_field_vec!(&biz_activitys, id);
        let names = rbatis::make_table_field_vec!(&biz_activitys, name);
        let map_id = rbatis::make_table_field_map!(&biz_activitys, id);
        let map_name = rbatis::make_table_field_map!(&biz_activitys, name);
        let r = rb
            .fetch_list_by_column::<Option<BizActivity>, _>("id", &ids)
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }
}
