use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::dark_std::sync::SyncVec;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::intercept_log::LogInterceptor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, crud, Error, RBatis};
use rbs::Value;
use std::sync::Arc;
use std::time::Duration;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
}
crud!(Activity {});

/// control log level or close log
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    //default rb.intercepts[0] = LogInterceptor{};
    let rb = RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();

    //insert to 0, will be [DisableLogIntercept{},LogInterceptor{}]
    rb.intercepts
        .insert(0, Arc::new(DisableLogIntercept::default()));

    let intercept: &DisableLogIntercept = rb.get_intercept().unwrap();
    intercept.skip_sql.push("delete from".to_string());

    //will not show log
    let _r = Activity::delete_by_column(&rb, "id", "1").await;

    log::logger().flush();
    println!("this is no log print by 'DisableLogIntercept'");
}

#[derive(Debug, Default)]
pub struct DisableLogIntercept {
    pub skip_sql: SyncVec<String>,
}

#[async_trait]
impl Intercept for DisableLogIntercept {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        for x in &self.skip_sql {
            if sql.contains(x) {
                //return Ok(false) will be skip next Intercept!
                return Ok(Some(false));
            }
        }
        Ok(Some(true))
    }

    async fn after(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        for x in &self.skip_sql {
            if sql.contains(x) {
                //return Ok(false) will be skip next Intercept!
                return Ok(Some(false));
            }
        }
        Ok(Some(true))
    }
}
