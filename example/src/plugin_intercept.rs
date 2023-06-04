pub mod init;
use crate::init::init_db;
use rbatis::intercept::{Intercept};
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, Error};
use rbs::Value;
use serde_json::json;
use rbatis::executor::Executor;

/// Logic delete： The deletion statement changes to the modification of flag, and the query statement filters flag with additional conditions
pub struct LogicDeletePlugin {}

impl Intercept for LogicDeletePlugin {
    fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
    ) -> Result<(), Error> {
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
        Ok(())
    }
}

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

crud!(BizActivity {});

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    rb.intercepts.push(Box::new(LogicDeletePlugin {}));
    let r = BizActivity::delete_by_column(&mut rb.clone(), "id", "1").await;
    println!("{}", json!(r));
    let record = BizActivity::select_by_column(&mut rb.clone(), "id", "1").await;
    println!("{}", json!(record));
    log::logger().flush();
}
