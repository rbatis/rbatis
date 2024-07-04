use std::sync::Arc;
use log::LevelFilter;
use serde_json::json;
use rbatis::{async_trait, crud, Error, RBatis};
use rbatis::dark_std::defer;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::table_sync::SqliteTableMapper;
use rbs::Value;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

crud!(Activity {});
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
    defer!(|| {
        log::logger().flush();
    });
    let rb_read = read_rb().await;
    let rb = write_rb().await;
    rb.intercepts.push(Arc::new(ReadWriteIntercept {
        read: rb_read,
    }));

    let table = Activity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("2".to_string()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };

    let data = Activity::insert(&rb, &table).await;
    println!("insert = {}", json!(data));

    let data = Activity::select_by_column(&rb, "id", "2").await;
    println!("select_in_column = {}", json!(data));
}

async fn read_rb() -> RBatis {
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite_read.db",
    )
        .unwrap();
    // table sync done
    sync_table(&rb).await;
    rb
}

async fn write_rb() -> RBatis {
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite_write.db",
    )
        .unwrap();
    // table sync done
    sync_table(&rb).await;
    rb
}

async fn sync_table(rb: &RBatis) {
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _ = RBatis::sync(
        &rb.acquire().await.unwrap(),
        &SqliteTableMapper {},
        &Activity {
            id: Some(String::new()),
            name: Some(String::new()),
            pc_link: Some(String::new()),
            h5_link: Some(String::new()),
            pc_banner_img: Some(String::new()),
            h5_banner_img: Some(String::new()),
            sort: Some(String::new()),
            status: Some(0),
            remark: Some(String::new()),
            create_time: Some(DateTime::now()),
            version: Some(0),
            delete_flag: Some(0),
        },
        "activity",
    )
        .await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);
}


#[derive(Debug)]
pub struct ReadWriteIntercept {
    read: RBatis,
}

#[async_trait]
impl Intercept for ReadWriteIntercept {
    async fn before(&self, _task_id: i64, _rb: &dyn Executor, sql: &mut String, args: &mut Vec<Value>, result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>) -> Result<Option<bool>, Error> {
        if sql.trim().starts_with("select") {
            println!("-------------------------------------------------------run on read database------------------------------------------------------------");
            let conn = self.read.acquire().await?;
            let r = conn.query(&sql.clone(), args.clone()).await;
            match r {
                Ok(r) => {
                    match result {
                        ResultType::Exec(exec) => {
                            let result = rbatis::decode(r);
                            *exec = Ok(result?);
                        }
                        ResultType::Query(q) => {
                            let result = rbatis::decode(r);
                            *q = Ok(result?);
                        }
                    }
                }
                Err(e) => {
                    match result {
                        ResultType::Exec(exec) => {
                            *exec = Err(e);
                        }
                        ResultType::Query(q) => {
                            *q = Err(e);
                        }
                    }
                }
            }
            Ok(None)
        } else {
            println!("-------------------------------------------------------run on write database------------------------------------------------------------");
            Ok(Some(true))
        }
    }

    async fn after(&self, _task_id: i64, _rb: &dyn Executor, sql: &mut String, args: &mut Vec<Value>, result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>) -> Result<Option<bool>, Error> {
        if sql.trim().starts_with("select") {
            let conn = self.read.acquire().await?;
            let r = conn.query(&sql.clone(), args.clone()).await;
            match r {
                Ok(r) => {
                    match result {
                        ResultType::Exec(exec) => {
                            let result = rbatis::decode(r);
                            *exec = Ok(result?);
                        }
                        ResultType::Query(q) => {
                            let result = rbatis::decode(r);
                            *q = Ok(result?);
                        }
                    }
                }
                Err(e) => {
                    match result {
                        ResultType::Exec(exec) => {
                            *exec = Err(e);
                        }
                        ResultType::Query(q) => {
                            *q = Err(e);
                        }
                    }
                }
            }
            Ok(None)
        } else {
            Ok(Some(true))
        }
    }
}

