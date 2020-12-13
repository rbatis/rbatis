///test CRUD
#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::str::FromStr;

    use bigdecimal_::BigDecimal;
    use chrono::NaiveDateTime;
    use serde::Deserialize;
    use serde::Serialize;
    use uuid::Uuid;

    use rbatis::core::db::DBExecResult;
    use rbatis::core::Error;
    use rbatis::core::value::DateTimeNow;
    use rbatis::crud::{CRUD, CRUDEnable};
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rbatis_macro_driver::sql;

    ///
            ///Or another way to write it
    // #[crud_enable(table_name:biz_activity)]
    // #[crud_enable(id_name:id|id_type:String|table_name:biz_activity|table_columns:id,name,version,delete_flag)]
    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<NaiveDateTime>,
        pub version: Option<BigDecimal>,
        pub delete_flag: Option<i32>,
    }

    #[crud_enable]
    #[derive(Clone, Debug)]
    pub struct BizActivity2 {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<NaiveDateTime>,
        pub version: Option<BigDecimal>,
        pub delete_flag: Option<i32>,
    }

// (可选) 手动实现，不使用上面的derive(CRUDEnable),可重写table_name方法。手动实现能支持IDE智能提示
// impl CRUDEnable for BizActivity {
//     type IdType = String;
// }

    pub async fn init_rbatis() -> Rbatis {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

        // custom connection option(自定义连接参数)
        // let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
        // rb.link_cfg(&db_cfg,PoolOptions::new());

        // custom pool(自定义连接池)
        // let mut opt = PoolOptions::new();
        // opt.max_size = 20;
        // rb.link_opt("mysql://root:123456@localhost:3306/test", &opt).await.unwrap();
        return rb;
    }


    #[async_std::test]
    pub async fn test_save() {
        let rb = init_rbatis().await;
        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: Some("123".to_string()),
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: Some("1".to_string()),
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        };
        let r = rb.save("", &activity).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[async_std::test]
    pub async fn test_save_batch() {
        let rb = init_rbatis().await;
        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: Some("test_1".to_string()),
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        };
        let args = vec![activity.clone(), activity];
        let r = rb.save_batch("", &args).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }


    #[async_std::test]
    pub async fn test_remove_batch_by_id() {
        let mut rb = init_rbatis().await;
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let r = rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }


    #[async_std::test]
    pub async fn test_remove_by_id() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("delete_flag", 1, 0)));
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let r = rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[async_std::test]
    pub async fn test_fetch_by_id() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r = rb.fetch_by_id::<Option<BizActivity>>("", &"1".to_string()).await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }

    #[async_std::test]
    pub async fn test_update_by_wrapper() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));

        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        };

        let w = rb.new_wrapper().eq("id", "12312").check().unwrap();
        let r = rb.update_by_wrapper("", &activity, &w, false).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }


    #[async_std::test]
    pub async fn test_update_by_id() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));

        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        };
        let r = rb.update_by_id("", &activity).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[async_std::test]
    pub async fn test_fetch_by_wrapper() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let w = rb.new_wrapper().eq("id", "1").check().unwrap();
        let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
        println!("is_some:{:?}", r);
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[async_std::test]
    pub async fn test_list_by_wrapper() {
        let rb = init_rbatis().await;
        let w = rb.new_wrapper()
            .order_by(true, &["id"]).check().unwrap();
        let r: Vec<BizActivity> = rb.list_by_wrapper("", &w).await.unwrap();
        println!("is_some:{:?}", r);
    }

    #[async_std::test]
    pub async fn test_fetch_page_by_wrapper() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));

        let w = rb.new_wrapper()
            .like("name", "test")
            //.order_by(false, &["create_time"])
            .check().unwrap();
        let r: Page<BizActivity> = rb.fetch_page_by_wrapper("", &w, &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }

    #[async_std::test]
    pub async fn test_list() {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r: Vec<BizActivity> = rb.list("").await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }


    lazy_static! {
     static ref RB:Rbatis=Rbatis::new();
   }

    /// RB是本地依赖Rbatis引用的名称,例如  dao::RB, com::xxx::RB....都可以
    /// 第二个参数是标准的驱动sql，注意对应数据库参数mysql为？,pg为$1...
    /// 宏会自动转换函数为  pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}
    ///
    #[sql(RB, "select * from biz_activity where id = ?")]
    fn select(name: &str) -> BizActivity {}

    /// Use static RB ref
    /// 使用全局变量例子
    #[py_sql(RB, "select * from biz_activity where id = #{name}
                  if name != '':
                    and name != #{name}")]
    fn py_select(name: &str) -> Option<BizActivity> {}

    /// Use Arg rbatis ref
    /// 使用参数变量例子
    #[py_sql(rbatis, "select * from biz_activity where id = #{name}
                  //注释信息
                  if name != '':
                    and name != #{name}")]
    fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = select("1").await.unwrap();
        println!("{:?}", a);
    }

    #[async_std::test]
    pub async fn test_macro_py_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select("1").await.unwrap();
        println!("{:?}", a);
        // use arg value
        let rbatis = Rbatis::new();
        rbatis.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select_rb(&rbatis, "1").await.unwrap();
        println!("{:?}", a);
    }


    #[py_sql(rbatis, "SELECT a1.name as name,a2.create_time as create_time
                      FROM test.biz_activity a1,biz_activity a2
                      WHERE a1.id=a2.id
                      AND a1.name=#{name}")]
    fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<BizActivity>> {}

    #[async_std::test]
    pub async fn test_join() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let results = join_select(&RB, "test").await.unwrap();
        println!("data: {:?}", results);
    }

    #[test]
    pub fn test_raw_identifiers() {
        use rbatis::crud::CRUDEnable;
        #[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
        pub struct BizActivity {
            pub id: Option<String>,
            // pub type: Option<String>, // type is a keyword, so need to named `r#type`.
            pub r#type: Option<String>,
        }

        assert_eq!("id,type".to_string(), BizActivity::table_columns());
    }

    /// Use py_select_page
   /// 使用分页宏例子
    #[py_sql(RB, "select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    fn py_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select_page(&PageRequest::new(1, 10), "test").await.unwrap();
        println!("{:?}", a);
    }


    /// Use sql_select_page
   /// 使用分页宏例子
    #[sql(RB, "select * from biz_activity where delete_flag = 0 and name = ?")]
    fn sql_select_page(page_req: &PageRequest, name: &str) -> Page<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_sql_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = sql_select_page(&PageRequest::new(1, 10), "test").await.unwrap();
        println!("{:?}", a);
    }

    #[py_sql(RB, "insert into biz_activity(
                  trim ',': for key,item in arg: if item != null:
                     ${key},
                  ) VALUES (
                  trim ',': for v in arg: if v != null:
                      #{v},
                  )   ")]
    fn py_insert(arg: &BizActivity) -> DBExecResult {}

    #[async_std::test]
    pub async fn test_py_insert() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_insert(&BizActivity {
            id: Some("12312".to_string()),
            name: Some("123".to_string()),
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: Some("1".to_string()),
            status: Some(1),
            remark: None,
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        }).await.unwrap();
        println!("{:?}", a);
    }

    #[async_std::test]
    pub async fn test_macro_sql_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();

        #[py_sql(RB, "select * from biz_activity where delete_flag = #{del}")]
        fn select_activities(del: i32) -> Vec<BizActivity> {}

        let ret = select_activities(1).await;

        log::info!("result is : {:?}", ret);
    }

    #[async_std::test]
    pub async fn test_pg_uuid() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("postgres://postgres:123456@localhost:5432/postgres").await.unwrap();
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct BizUuid {
            pub id: Option<Uuid>,
            pub name: Option<String>,
        }
        impl CRUDEnable for BizUuid {
            type IdType = Uuid;

            fn id_name() -> String {
                "id".to_string()
            }

            fn format_chain() -> HashMap<String, String> {
                let mut m = HashMap::new();
                m.insert("id".to_string(), "{}::uuid".to_string());
                m
            }
        }

        //make table
        rb.exec("", "CREATE TABLE biz_uuid( id uuid, name VARCHAR, PRIMARY KEY(id));").await;
        //rb.exec("", "INSERT INTO biz_uuid (id,name) VALUES ('df07fea2-b819-4e05-b86d-dfc15a5f52a9','uuid')").await;
        let uuid = Uuid::from_str("df07fea2-b819-4e05-b86d-dfc15a5f52a9").unwrap();
        rb.save("", &BizUuid { id: Some(uuid), name: Some("test".to_string()) }).await;
        let w = rb.new_wrapper().push_sql("id = 'df07fea2-b819-4e05-b86d-dfc15a5f52a9'::uuid").check().unwrap();
        let data: BizUuid = rb.fetch_by_wrapper("", &w).await.unwrap();
        println!("{:?}", data);

        let uuid=Uuid::from_str("df07fea2-b819-4e05-b86d-dfc15a5f52a9").unwrap();
        rb.remove_by_id::<BizUuid>("",&uuid).await;
    }
}