#[macro_use]
extern crate rbatis;

use log::LevelFilter;
use rbatis::{RBatis, table_sync};
use rbatis::dark_std::defer;
use rbatis::table_sync::SqliteTableMapper;
use rbs::{to_value};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: Option<u64>,
    pub name: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Option<u64>,
    //json support json object/array
    pub account1: Account,
    //json support json object/array
    pub account2: Vec<Account>,
}

crud!(User {});

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(||{log::logger().flush();});

    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    create_table(&rb).await;
    let user = User {
        id: Some(1),
        account1: Account {
            id: Some(2),
            name: Some("xxx".to_string()),
        },
        account2: vec![Account {
            id: Some(2),
            name: Some("xxx".to_string()),
        }],
    };

    let v = User::insert(&rb.clone(), &user).await;
    println!("insert:{:?}", v);

    let users = User::select_by_column(&rb.clone(), "id", 1).await;
    println!("select:{}", to_value!(users));
}

async fn create_table(rb: &RBatis) {
    fast_log::LOGGER.set_level(LevelFilter::Off);
    defer!(||{
         fast_log::LOGGER.set_level(LevelFilter::Info);
    });
    let table = to_value!{
        "id":"INTEGER PRIMARY KEY AUTOINCREMENT",
        "account1":"JSON",
        "account2":"JSON",
    };
    let conn = rb.acquire().await.unwrap();
    _ = table_sync::sync(&conn, &SqliteTableMapper {}, to_value!(&table), "user").await;
}
