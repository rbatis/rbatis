pub mod model;

use std::thread::sleep;
use std::time::Duration;
use rbatis::intercept::SqlIntercept;
use rbatis::{crud, Error, Rbatis};
use rbs::Value;
use crate::model::{BizActivity, init_db};

pub struct LogicDeletePlugin {}

impl SqlIntercept for LogicDeletePlugin {
    fn do_intercept(&self, _rb: &Rbatis, sql: &mut String, _args: &mut Vec<Value>, _is_prepared_sql: bool) -> Result<(), Error> {
        if sql.contains("delete from "){
            let table_name = sql[sql.find("from").unwrap_or(0) + 4..sql.find("where").unwrap_or(0)].trim();
            println!("table_name => {}",table_name);
            println!("befor => {}",sql);
            *sql = sql.replace(&format!("delete from {}", table_name), &format!("update {} set delete_flag = 0 ", table_name));
            println!("after => {}",sql);
        }
        Ok(())
    }
}

crud!(BizActivity {});

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let mut rb = init_db().await;
    rb.set_sql_intercepts(vec![Box::new(LogicDeletePlugin {})]);
    let r = BizActivity::delete_by_column(&mut rb.clone(), "id", "1").await;
    println!("{:?}", r);
    sleep(Duration::from_secs(1));
}