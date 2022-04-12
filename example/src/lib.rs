#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;

use std::fs::{create_dir_all, File};
use std::io::Read;
use rbatis::rbatis::Rbatis;

mod crud;
mod dyn_table_name;
mod postgres_format;
mod get_fields;
mod plugin_intercept;
mod plugin_page;
mod plugin_logic_del;
mod macro_proc_pysql;
mod macro_proc_htmlsql;
mod macro_proc_htmlsql_custom_func;
mod macro_proc_rawsql;
mod raw_driver;
mod column_keyword;
mod macro_tool;
mod transaction;
mod plugin_snowflake;
mod plugin_object_id;
mod wrapper;
mod wrapper_macro;


/// this is table model(see ../database.sql)
#[crud_table]
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
    pub create_time: Option<rbatis::DateTimeNative>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

/// make a sqlite-rbatis
pub async fn init_sqlite() -> Rbatis {
    init_sqlite_path("../").await
}

/// make a sqlite-rbatis
pub async fn init_sqlite_path(root_path:&str) -> Rbatis {
    if File::open(format!("{}target/sqlite.db",root_path)).is_err() {
        create_dir_all(format!("{}target/",root_path));
        let f = File::create(format!("{}target/sqlite.db",root_path)).unwrap();
        drop(f);
    }
    fast_log::init(fast_log::config::Config::new().console());


    // custom connection option
    // //mysql
    // // let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
    // let db_cfg=DBConnectOption::from("sqlite://../target/sqlite.db")?;
    // rb.link_cfg(&db_cfg,PoolOptions::new());

    // custom pool
    // let mut opt = PoolOptions::new();
    // opt.max_size = 20;
    // rb.link_opt("sqlite://../target/sqlite.db", &opt).await.unwrap();

    // init rbatis
    let rb = Rbatis::new();
    rb.link(&format!("sqlite://{}target/sqlite.db",root_path))
        .await
        .unwrap();

    // run sql create table
    let mut f = File::open(format!("{}example/table_sqlite.sql",root_path)).unwrap();
    let mut sql = String::new();
    f.read_to_string(&mut sql).unwrap();
    rb.exec(&sql, vec![]).await;

    return rb;
}