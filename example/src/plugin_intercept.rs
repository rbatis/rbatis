pub mod model;

use crate::model::{init_db, BizActivity};
use rbatis::intercept::SqlIntercept;
use rbatis::{crud, Error, Rbatis};
use rbs::Value;
use std::time::Duration;
use log::Log;

/// Logic delete： The deletion statement changes to the modification of flag, and the query statement filters flag with additional conditions
pub struct LogicDeletePlugin {}

impl SqlIntercept for LogicDeletePlugin {
    fn do_intercept(
        &self,
        _rb: &Rbatis,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _is_prepared_sql: bool,
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

crud!(BizActivity {});

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    rb.sql_intercepts.push(Box::new(LogicDeletePlugin {}));
    let r = BizActivity::delete_by_column(&mut rb.clone(), "id", "1").await;
    println!("{:?}", r);
    let record = BizActivity::select_by_column(&mut rb.clone(), "id", "1").await;
    println!("{:?}", record);
    log::logger().flush();
}
