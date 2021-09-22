///this is postgres  database  !
#[cfg(test)]
mod test {
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;
    use serde_json::json;
    use uuid::Uuid;
    use rbatis::executor::Executor;

    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct P {
        pub id: i32,
        ///Postgres Point is not currently supported, but Vec is used instead
        pub po: Vec<u8>,
    }

    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct P2 {
        pub id: Uuid,
        ///Postgres Point is not currently supported, but Vec is used instead
        pub po: Vec<u8>,
    }

    /// you may should use pg database! this is docker command for example:
    /// docker run -d --name postgres  -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres
    #[tokio::test]
    pub async fn test_select_point() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("postgres://postgres:123456@localhost:5432/postgres")
            .await
            .unwrap();
        //create table
        rb.exec(r#"CREATE TABLE "public"."p" ( "id" int4, "po" point);"#, vec![])
            .await;
        //insert point
        rb.exec(
            "insert into p (id,po) values (1,Point($1,$2))",
            vec![json!(1), json!(2)],
        )
            .await
            .unwrap();
        //query table
        let v: Vec<P> = rb
            .fetch_list_by_wrapper(rb.new_wrapper_table::<P>().eq("id", 1))
            .await
            .unwrap();
    }
}
