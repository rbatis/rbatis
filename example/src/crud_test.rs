///test CRUD
#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde::Serialize;

    use rbatis::crud::{CRUD, CRUDEnable};
    use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rbatis_core::Error;
    use rbatis_macro_driver::sql;
    use chrono::NaiveDateTime;
    use bigdecimal_::BigDecimal;
    use rbatis_core::value::DateTimeNow;

    #[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
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

// (可选) 手动实现，不使用上面的derive(CRUDEnable),可重写table_name方法。手动实现能支持IDE智能提示
// impl CRUDEnable for BizActivity {
//     type IdType = String;
// }

    pub async fn init_rbatis() -> Rbatis {
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

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
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
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
    /// 宏会自动转换函数为  pub async fn select(name: &str) -> rbatis_core::Result<BizActivity> {}
    ///
    #[sql(RB, "select * from biz_activity where id = ?")]
    fn select(name: &str) -> BizActivity {}

    /// Use static RB ref
    /// 使用全局变量例子
    #[py_sql(RB, "select * from biz_activity where id = #{name}
                  if name != '':
                    and name=#{name}")]
    fn py_select(name: &str) -> Option<BizActivity> {}

    /// Use Arg rbatis ref
    /// 使用参数变量例子
    #[py_sql(rbatis, "select * from biz_activity where id = #{name}
                  if name != '':
                    and name=#{name}")]
    fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_select() {
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = select("1").await.unwrap();
        println!("{:?}", a);
    }

    #[async_std::test]
    pub async fn test_macro_py_select() {
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
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


    /// join method,you can use
    /// JOIN:
    /// LEFT JOIN:
    /// RIGHT JOIN:
    /// FULL JOIN:
    #[py_sql(rbatis, "SELECT a1.name as name,a2.create_time as create_time
                      FROM test.biz_activity a1,biz_activity a2
                      WHERE a1.id=a2.id
                      AND a1.name=#{name}")]
    fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<BizActivity>> {}

    #[async_std::test]
    pub async fn test_join() {
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let results = join_select(&RB, "test").await.unwrap();
        println!("data: {:?}", results);
    }

    #[test]
    pub fn test_raw_identifiers() {
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
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
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
        fast_log::init_log("requests.log", 1000,log::Level::Info,true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = sql_select_page(&PageRequest::new(1, 10), "test").await.unwrap();
        println!("{:?}", a);
    }
}