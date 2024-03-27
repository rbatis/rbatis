use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, crud, Error, RBatis};
use rbs::Value;
use serde_json::json;
use std::sync::Arc;
use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::table_sync::SqliteTableMapper;

/// Logic delete： The deletion statement changes to the modification of flag, and the query statement filters flag with additional conditions
#[derive(Debug)]
pub struct LogicDeleteIntercept {}

#[async_trait]
impl Intercept for LogicDeleteIntercept {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<bool, Error> {
        if sql.contains("delete from ") {
            let table_name =
                sql[sql.find("from").unwrap_or(0) + 4..sql.find("where").unwrap_or(0)].trim();
            println!("[LogicDeletePlugin] before=> {}", sql);
            *sql = sql.replace(
                &format!("delete from {}", table_name),
                &format!("update {} set delete_flag = 1 ", table_name),
            );
            println!("[LogicDeletePlugin] after=> {}", sql);
        } else if sql.contains("select ") && sql.contains(" where ") {
            println!("[LogicDeletePlugin] before=> {}", sql);
            sql.push_str(" and delete_flag = 0 ");
            println!("[LogicDeletePlugin] after=> {}", sql);
        }
        Ok(true)
    }
}

/// table
#[derive(serde::Serialize, serde::Deserialize)]
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
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(||{log::logger().flush();});
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // table sync done
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _=RBatis::sync(&rb.acquire().await.unwrap(), &SqliteTableMapper{}, &Activity{
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
    }, "activity").await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);

    rb.intercepts.push(Arc::new(LogicDeleteIntercept {}));

    //get intercept
    let intercept = rb.get_intercept::<LogicDeleteIntercept>().unwrap();
    println!("intercept name = {}", intercept.name());
    //query
    let r = Activity::delete_by_column(&rb, "id", "1").await;
    println!("{}", json!(r));
    let record = Activity::select_by_column(&rb, "id", "1").await;
    println!("{}", json!(record));
}
