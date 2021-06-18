#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;

    use rbatis::crud::Fields;

    /// This example shows a table collection  to an id array
    #[tokio::test]
    pub async fn test_fetch_by_ids() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let biz_activitys = rb.fetch_list::<BizActivity>().await.unwrap();

        //to_ids() support HashSet.to_ids(),Vec.to_ids(),Array.to_ids(),HashMap.to_ids(),LinkedList.to_ids()，BtreeMap.to_ids()....
        let ids = biz_activitys.to_fields::<serde_json::Value>("id");

        let r = rb
            .fetch_list_by_column::<Option<BizActivity>, _>("id", &ids)
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }
}
