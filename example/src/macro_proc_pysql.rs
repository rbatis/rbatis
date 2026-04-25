use rbatis::dark_std::defer;
use rbatis::executor::Executor;
use rbatis::pysql;
use rbatis::rbatis_codegen::IntoSql;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::ExecResult;
use rbatis::{py_sql, Error, RBatis};
use serde_json::json;

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

#[py_sql(
    "`select * from activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}"
)]
async fn py_select(rb: &dyn Executor, name: &str, ids: &[i32]) -> Result<Vec<Activity>, Error> {
    impled!()
}

#[py_sql(
    "`delete from activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}"
)]
async fn py_exec(rb: &dyn Executor, name: &str, ids: &[i32]) -> Result<ExecResult, Error> {
    impled!()
}

pysql!(py_exec2(rb: &dyn Executor, name: &str, ids: &[i32]) -> Result<ExecResult, Error> =>
    "`delete from activity where delete_flag = 0`
                   if name != '':
                     ` and name=#{name}`
                   if !ids.is_empty():
                     ` and id in `
                     ${ids.sql()}" );

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    defer!(|| {
        log::logger().flush();
    });
    //use static ref
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();
   
    let a = py_select(&rb, "", &[1, 2, 3]).await.unwrap();
    println!(">>>>>>>>>>>> {}", json!(a));

    let a = py_exec(&rb, "", &[1, 2, 3]).await.unwrap();
    println!(">>>>>>>>>>>> {}", json!(a));

    let a = py_exec2(&rb, "", &[1, 2, 3]).await.unwrap();
    println!(">>>>>>>>>>>> {}", json!(a));
}
