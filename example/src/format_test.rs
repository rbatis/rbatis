#[cfg(test)]
mod test {
    use rbatis::rbatis::Rbatis;
    use uuid::Uuid;
    use std::str::FromStr;
    use rbatis::crud::CRUD;

    //'formats_pg' use postgres format
//'id' ->  table column 'id'
//'{}::uuid' -> format data str
    #[crud_enable(formats_pg: "id:{}::uuid")]
    #[derive(Clone, Debug)]
    pub struct BizUuid {
        pub id: Option<Uuid>,
        pub name: Option<String>,
    }

    #[async_std::test]
    pub async fn test_postgres_uuid() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("postgres://postgres:123456@localhost:5432/postgres").await.unwrap();

        let uuid = Uuid::from_str("df07fea2-b819-4e05-b86d-dfc15a5f52a9").unwrap();
        //create table
        rb.exec("", "CREATE TABLE biz_uuid( id uuid, name VARCHAR, PRIMARY KEY(id));").await;
        //insert table
        rb.save("", &BizUuid { id: Some(uuid), name: Some("test".to_string()) }).await;
        //update table
        rb.update_by_id("", &BizUuid { id: Some(uuid.clone()), name: Some("test_updated".to_string()) }).await;
        //query table
        let data: BizUuid = rb.fetch_by_id("", &uuid).await.unwrap();
        println!("{:?}", data);
        //delete table
        rb.remove_by_id::<BizUuid>("", &uuid).await;
    }
}