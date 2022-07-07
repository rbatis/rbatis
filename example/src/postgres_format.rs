///this is postgres  database  !
#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::plugin::intercept::RbatisLogFormatSqlIntercept;
    use rbatis::rbatis::Rbatis;
    use std::str::FromStr;
    use uuid::Uuid;
    use rbatis::DateTimeNative;

    //'formats_pg' use postgres format
    //'id' ->  table column 'id'
    //'{}::uuid' -> format data str
    // '\\,'   ->   This is an escape symbol
    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct BizUuid {
        pub id: Option<String>,
        pub name: Option<String>,
    }

    /// you may should use pg database! this is docker command for example:
    /// docker run -d --name postgres  -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres
    ///
    #[tokio::test]
    pub async fn test_postgres_uuid() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb = Rbatis::new();
        rb.link("postgres://postgres:123456@localhost:5432/postgres")
            .await
            .unwrap();
        
        rb.save(&BizUuid{
            id: Some("6".to_string()),
            name: None
        },&[]).await.unwrap();

        // let uuid = Uuid::from_str("df07fea2-b819-4e05-b86d-dfc15a5f52a9").unwrap();
        // //create table
        // rb.exec("DROP TABLE biz_uuid;", vec![]).await;
        // rb.exec("CREATE TABLE biz_uuid( id uuid, name VARCHAR, PRIMARY KEY(id));", vec![])
        //     .await;
        // //insert table
        // rb.save(
        //     &BizUuid {
        //         id: Some(uuid),
        //         name: Some("test".to_string()),
        //     },
        //     &[],
        // )
        //     .await;
        // //update table
        // rb.update_by_column(
        //     "id",
        //     &mut BizUuid {
        //         id: Some(uuid.clone()),
        //         name: Some("test_updated".to_string()),
        //     },
        // )
        //     .await;
        // //query table
        // let data: BizUuid = rb.fetch_by_column("id", &uuid).await.unwrap();
        // println!("{:?}", data);
        // //delete table
        // rb.remove_by_column::<BizUuid, _>("id", &uuid).await;
    }

    /// Formatting precompiled SQL
    ///
    /// [] Exec  ==> insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag)
    /// values (?,?,?,?,?,?,?,?,?,?,?,?)
    ///
    /// into
    ///
    /// [rbatis] [] [format_sql]insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag)
    /// values ("12312","12312",null,null,null,null,"1",1,null,"2021-03-10T20:34:47.432751100",1,1)
    ///
    #[tokio::test]
    pub async fn test_show_format_sql() {
        fast_log::init(fast_log::config::Config::new().console());
        let mut rb = Rbatis::new();
        //RbatisLogFormatSqlIntercept will show Formatted SQL(no precompiled symbols)
        rb.add_sql_intercept(RbatisLogFormatSqlIntercept {});
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        use rbatis::core::value::DateTimeNow;

        let activity = BizActivity {
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
            version: Some(1),
            delete_flag: Some(1),
        };
        rb.remove_by_column::<BizActivity, _>("id", activity.id.as_ref().unwrap())
            .await;
        let r = rb.save(&activity, &[]).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }
}
