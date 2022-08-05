use rbatis::rbatis::Rbatis;
use std::fs::{create_dir_all, File};
use std::io::Read;
use log::LevelFilter;
use serde::{Serialize, Deserialize};
use rbdc::datetime::FastDateTime;
use rbdc_sqlite::driver::SqliteDriver;

/// this is table model(see ../database.sql)
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

/// make a sqlite-rbatis
pub async fn init_sqlite() -> Rbatis {
    init_sqlite_path("").await
}

/// make a sqlite-rbatis
pub async fn init_sqlite_path(path: &str) -> Rbatis {
    //first init log carte
    fast_log::init(fast_log::config::Config::new().console());

    // new rbatis
    let rb = Rbatis::new();

    // // mysql custom connection option
    // // let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
    // let db_cfg= DBConnectOption::from("sqlite://../target/sqlite.db")?;
    // rb.link_cfg(&db_cfg,DBPoolOptions::new());
    //
    // // custom pool
    // let mut opt = DBPoolOptions::new();
    // opt.max_size = 20;
    // rb.link_opt("sqlite://../target/sqlite.db", &opt).await.unwrap();

    //create sqlite file
    if File::open(format!("{}target/sqlite.db", path)).is_err() {
        create_dir_all(format!("{}target/", path));
        let f = File::create(format!("{}target/sqlite.db", path)).unwrap();
        drop(f);
    }
    rb.link(SqliteDriver {}, &format!("sqlite://{}target/sqlite.db", path))
        .await
        .unwrap();

    // run sql create table
    let mut f = File::open(format!("{}example/table_sqlite.sql", path)).unwrap();
    let mut sql = String::new();
    f.read_to_string(&mut sql).unwrap();

    fast_log::LOGGER.set_level(LevelFilter::Off);
    rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(LevelFilter::Info);
    return rb;
}
