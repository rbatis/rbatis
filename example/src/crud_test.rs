use chrono::{DateTime, Utc};
use fast_log::log::RuntimeType;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use rbatis::crud::{CRUD, CRUDEnable};
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use rbatis_core::db::{PoolOptions, DriverType};
use rbatis_core::Error;
use rbatis_core::types::chrono::NaiveDateTime;
use rbatis_core::value::DateTimeNow;

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
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

/// 必须实现 CRUDEntity接口，如果表名 不正确，可以重写 fn table_name() -> String 方法！
impl CRUDEnable for BizActivity {
    type IdType = String;
}

pub async fn init_rbatis() -> Rbatis<'static> {
    fast_log::log::init_log("requests.log", &RuntimeType::Std);
    let rb = Rbatis::new();
    rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();

    // or use opt custom pool
    // let mut opt = PoolOptions::new();
    // opt.max_size = 20;
    // rb.link_opt("mysql://root:123456@localhost:3306/test", &opt).await.unwrap();
    return rb;
}

#[test]
pub fn test_save() {
    async_std::task::block_on(async {
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
            version: Some(1),
            delete_flag: Some(1),
        };
        let r = rb.save("", &activity).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}

#[test]
pub fn test_save_batch() {
    async_std::task::block_on(async {
        let rb = init_rbatis().await;
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
            version: Some(1),
            delete_flag: Some(1),
        };
        let args = vec![activity.clone(), activity];
        let r = rb.save_batch("", &args).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}


#[test]
pub fn test_remove_batch_by_id() {
    async_std::task::block_on(async {
        let mut rb = init_rbatis().await;
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let r = rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}


#[test]
pub fn test_remove_by_id() {
    async_std::task::block_on(async {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let r = rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}

#[test]
pub fn test_update_by_wrapper() {
    async_std::task::block_on(async {
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
            version: Some(1),
            delete_flag: Some(1),
        };

        let w = rb.new_wrapper().eq("id", "12312").check().unwrap();
        let r = rb.update_by_wrapper("", &activity, &w).await;
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}


#[test]
pub fn test_update_by_id() {
    async_std::task::block_on(async {
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
    async_std::task::block_on(async {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let w = rb.new_wrapper().eq("id", "1").check().unwrap();
        let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
        println!("is_some:{:?}", r);
        if r.is_err() {
            println!("{}", r.err().unwrap().to_string());
        }
    });
}


#[test]
pub fn test_fetch_page_by_wrapper() {
    async_std::task::block_on(async {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));

        let w = rb.new_wrapper()
            .eq("delete_flag", 1)
            .check().unwrap();
        let r: Page<BizActivity> = rb.fetch_page_by_wrapper("", &w, &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    });
}

#[test]
pub fn test_list() {
    async_std::task::block_on(async {
        let mut rb = init_rbatis().await;
        //设置 逻辑删除插件
        rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
        let r: Vec<BizActivity> = rb.list("").await.unwrap();
        println!("{}", serde_json::to_string(&r).unwrap());
    });
}