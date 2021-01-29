#[cfg(test)]
mod test {
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;

    use chrono::{DateTime, Utc};
    use serde::de::DeserializeOwned;
    use serde::Deserialize;
    use serde::Serialize;

    use rbatis::core::Error;
    use rbatis::crud::{CRUD, CRUDEnable, Id, Ids};
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rbatis::wrapper::Wrapper;

    #[derive(Serialize, Deserialize, Clone, Debug)]
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
        pub create_time: Option<String>,
        pub version: Option<i32>,
        pub delete_flag: Option<i32>,
    }

    /// 必须实现 CRUDEntity接口，如果表名 不正确，可以重写 fn table_name() -> String 方法！
    impl CRUDEnable for BizActivity {
        type IdType = String;
    }

    impl Id for BizActivity {
        type IdType = String;

        fn get_id(&self) -> Option<Self::IdType> {
            self.id.clone()
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BizActivityNoDel {
        pub id: Option<String>,
        pub name: Option<String>,
    }

    impl CRUDEnable for BizActivityNoDel {
        type IdType = String;
        fn table_name() -> String {
            "biz_activity".to_string()
        }
    }

    #[test]
    pub fn test_ids() {
        let vec = vec![BizActivity {
            id: Some("12312".to_string()),
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(1),
            remark: None,
            create_time: Some("2020-02-09 00:00:00".to_string()),
            version: Some(1),
            delete_flag: Some(1),
        }];
        let ids = vec.to_ids();
        println!("{:?}", ids);
    }

    #[test]
    pub fn test_save() {
        rbatis::core::runtime::block_on(async {
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
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };

            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();
            let r = rb.save("", &activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_save_batch() {
        rbatis::core::runtime::block_on(async {
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
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };
            let args = vec![activity.clone(), activity];

            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let rb = Rbatis::new();
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();
            let r = rb.save_batch("", &args).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_remove_batch_by_id() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
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
        });
    }

    #[test]
    pub fn test_remove_by_id() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();
            let r = rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_update_by_wrapper() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

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
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };

            let w = Wrapper::new(&rb.driver_type().unwrap())
                .eq("id", "12312")
                .check()
                .unwrap();
            let r = rb.update_by_wrapper("", &activity, &w, false).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_update_by_id() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

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
                create_time: Some("2020-02-09 00:00:00".to_string()),
                version: Some(1),
                delete_flag: Some(1),
            };
            let r = rb.update_by_id("", &activity).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_by_wrapper() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap())
                .eq("id", "12312")
                .check()
                .unwrap();
            let r: Result<BizActivity, Error> = rb.fetch_by_wrapper("", &w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_no_del() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap())
                .eq("id", "12312")
                .check()
                .unwrap();
            let r: Result<BizActivityNoDel, Error> = rb.fetch_by_wrapper("", &w).await;
            if r.is_err() {
                println!("{}", r.err().unwrap().to_string());
            }
        });
    }

    #[test]
    pub fn test_fetch_page_by_wrapper() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

            let w = Wrapper::new(&rb.driver_type().unwrap()).check().unwrap();
            let r: Page<BizActivity> = rb
                .fetch_page_by_wrapper("", &w, &PageRequest::new(1, 20))
                .await
                .unwrap();
            println!("{}", serde_json::to_string(&r).unwrap());
        });
    }

    #[test]
    fn test_insert_order() {
        rbatis::core::runtime::block_on(async {
            fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
            let mut rb = Rbatis::new();
            //设置 逻辑删除插件
            rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
            rb.link("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();

            let py_sql = r#"
                        update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag},
                        create_datetime=current_timestamp(), update_datetime=current_timestamp() where id=#{id}
                    "#;
            let (sql, args) = rb.runtime_py.eval(&rb.driver_type().unwrap(), py_sql, &mut serde_json::json!({"name":"name", "password":"ps_encode","sex": "sex", "phone": "phone", "flag":0, "id": "u.id"}), &rb.runtime_expr).unwrap();
            assert_eq!(sql, "update user set name=?, password=? ,sex=?, phone=?, delete_flag=?, create_datetime=current_timestamp(), update_datetime=current_timestamp() where id=?");
            assert_eq!(
                serde_json::json!(args).to_string(),
                serde_json::json!(["name", "ps_encode", "sex", "phone", 0, "u.id"]).to_string()
            );
        });
    }

    #[async_std::test]
    pub async fn test_list_by_wrapper() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        let mut rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut w = rb.new_wrapper();
        w = w.order_by(true, &["id"]);
        w = w.limit(50);
        println!("{}", w.sql);
        let b: Vec<BizActivity> = rb.list_by_wrapper("", &w).await.unwrap();
    }
}
