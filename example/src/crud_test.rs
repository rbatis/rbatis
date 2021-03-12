#[cfg(test)]
mod test {
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;

    use rbatis::core::Error;
    use rbatis::core::value::DateTimeNow;
    use rbatis::crud::CRUD;
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::plugin::snowflake::new_snowflake_id;
    use rbatis::plugin::version_lock::RbatisVersionLockPlugin;
    use rbatis::rbatis::Rbatis;

    ///Or another way to write it
    // #[crud_enable(table_name:biz_activity)]
    // #[crud_enable(id_name:"id"|id_type:"String"|table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
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

    impl Default for BizActivity {
        fn default() -> Self {
            Self {
                id: None,
                name: None,
                pc_link: None,
                h5_link: None,
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: None,
                remark: None,
                create_time: None,
                version: None,
                delete_flag: None,
            }
        }
    }

    // (可选) 手动实现，不使用上面的derive(CRUDTable),可重写table_name方法。手动实现能支持IDE智能提示
    // impl CRUDTable for BizActivity {
    //     type IdType = String;
    // }

    pub async fn init_rbatis() -> Rbatis {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        // custom connection option(自定义连接参数)
        // let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
        // rb.link_cfg(&db_cfg,PoolOptions::new());

        // custom pool(自定义连接池)
        // let mut opt = PoolOptions::new();
        // opt.max_size = 20;
        // rb.link_opt("mysql://root:123456@localhost:3306/test", &opt).await.unwrap();
        return rb;
    }

    #[tokio::test]
    pub async fn test_snow_flake() {
        let sn_id = new_snowflake_id();
        println!("id:{}", sn_id);
    }

    #[tokio::test]
    pub async fn test_save() {
        let rb = init_rbatis().await;
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
            create_time: Some(NaiveDateTime::now()),
            version: Some(BigDecimal::from(1)),
            delete_flag: Some(1),
        };
        rb.remove_by_id::<BizActivity>("", activity.id.as_ref().unwrap())
            .await;
        let r = rb.save("", &activity).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_save_batch() {
        let rb = init_rbatis().await;
        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: Some("test_1".to_string()),
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
        let mut activity2 = activity.clone();
        activity2.id = Some("12313".to_string());
        rb.remove_batch_by_id::<BizActivity>("", &["12312".to_string(), "12313".to_string()])
            .await;
        let args = vec![activity, activity2];
        let r = rb.save_batch("", &args).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_save_batch_slice() {
        let rb = init_rbatis().await;
        let activity = BizActivity {
            id: Some("12312".to_string()),
            name: Some("test_1".to_string()),
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
        let mut activity2 = activity.clone();
        activity2.id = Some("12313".to_string());
        let mut activity3 = activity.clone();
        activity3.id = Some("12314".to_string());
        rb.remove_batch_by_id::<BizActivity>("", &["12312".to_string(), "12313".to_string(), "12314".to_string()])
            .await;
        let args = vec![activity, activity2, activity3];
        let r = rb.save_batch_slice("", &args, 2).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_remove_batch_by_id() {
        let mut rb = init_rbatis().await;
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let r = rb
            .remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()])
            .await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_remove_by_id() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
            "delete_flag",
            1,
            0,
        )));
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let r = rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_fetch_by_id() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r = rb
            .fetch_by_id::<Option<BizActivity>>("", &"1".to_string())
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }

    #[tokio::test]
    pub async fn test_count_by_wrapper() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r = rb
            .fetch_count_by_wrapper::<BizActivity>("", &rb.new_wrapper())
            .await
            .unwrap();
        println!("count(1): {}", r);
    }

    #[tokio::test]
    pub async fn test_update_by_wrapper() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        rb.version_lock_plugin = Some(Box::new(RbatisVersionLockPlugin::new("version")));
        let mut activity = BizActivity {
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

        let w = rb.new_wrapper().eq("id", "12312");
        let r = rb.update_by_wrapper("", &mut activity, &w, false).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
        println!("new_version:{}", &activity.version.unwrap());
    }

    #[tokio::test]
    pub async fn test_update_by_id() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        //macro make object
        let mut activity = rbatis::make_table!(BizActivity{
            id: "12312".to_string(),
            status: 1,
            create_time: NaiveDateTime::now(),
            version: BigDecimal::from(1),
            delete_flag: 1,
        });
        // or you can make source struct
        // let mut activity = BizActivity {
        //     id: Some("12312".to_string()),
        //     name: None,
        //     pc_link: None,
        //     h5_link: None,
        //     pc_banner_img: None,
        //     h5_banner_img: None,
        //     sort: None,
        //     status: Some(1),
        //     remark: None,
        //     create_time: Some(NaiveDateTime::now()),
        //     version: Some(BigDecimal::from(1)),
        //     delete_flag: Some(1),
        // };
        let r = rb.update_by_id("", &mut activity).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_fetch_by_wrapper() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let w = rb.new_wrapper().eq("id", "1");
        let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
        println!("is_some:{:?}", r);
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    }

    #[tokio::test]
    pub async fn test_list_by_wrapper() {
        let rb = init_rbatis().await;
        let w = rb.new_wrapper().order_by(true, &["id"]);
        let r: Vec<BizActivity> = rb.fetch_list_by_wrapper("", &w).await.unwrap();
        println!("is_some:{:?}", r);
    }

    #[tokio::test]
    pub async fn test_fetch_page_by_wrapper() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));

        let w = rb
            .new_wrapper()
            .like("name", "test")
            //.order_by(false, &["create_time"])
            ;
        let r: Page<BizActivity> = rb
            .fetch_page_by_wrapper("", &w, &PageRequest::new(1, 20))
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }

    #[tokio::test]
    pub async fn test_list() {
        let mut rb = init_rbatis().await;
        //set logic plugin
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r: Vec<BizActivity> = rb.fetch_list("").await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    }

    #[test]
    pub fn test_is_debug() {
        let rb = Rbatis::new();
        println!("is debug: {}", rb.is_debug_mode());
    }
}
