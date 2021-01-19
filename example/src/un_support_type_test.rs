///this is postgres  database  !
#[cfg(test)]
mod test {
    use rbatis::crud::CRUD;
    use rbatis::rbatis::Rbatis;
    use serde_json::json;

    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct P {
        pub id: i32,
        ///Postgres Point is not currently supported, but Vec is used instead
        pub po: Vec<u8>,
    }

    #[async_std::test]
    pub async fn test_select_point() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("postgres://postgres:123456@localhost:5432/postgres")
            .await
            .unwrap();
        //create table
        rb.exec("", r#"CREATE TABLE "public"."p" ( "id" int4, "po" point);"#)
            .await;
        //insert point
        rb.exec_prepare(
            "",
            "insert into p (id,po) values (1,Point($1,$2))",
            &vec![json!(1), json!(2)],
        )
        .await
        .unwrap();
        //query table
        let v: Vec<P> = rb
            .list_by_wrapper("", &rb.new_wrapper_table::<P>().eq("id", 1))
            .await
            .unwrap();
    }
}
