use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::ExecResult;
use rbatis::{async_trait, crud, Action, Error, RBatis};
use rbs::Value;
use serde_json::json;
use std::sync::Arc;

/// Postgres insert sql returning id Intercept
#[derive(Debug)]
pub struct ReturningIdPlugin {}

#[async_trait]
impl Intercept for ReturningIdPlugin {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        if sql.contains("insert into") {
            let new_sql = format!("{} {}", sql, "returning id");
            let new_args = args.clone();
            match result {
                ResultType::Exec(exec_r) => {
                    let id = rb.query(&new_sql, new_args).await?;
                    let id: String = rbatis::decode(id)?;
                    let mut exec = ExecResult::default();
                    exec.last_insert_id = id.into();
                    *exec_r = Ok(exec);
                    Ok(Action::Return)
                }
                ResultType::Query(_) => Ok(Action::Next),
            }
        } else {
            Ok(Action::Next)
        }
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

//docker run -d --name postgres  -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres
#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(LevelFilter::Debug),
    );
    defer!(|| log::logger().flush());
    let rb = RBatis::new();
    rb.init(
        rbdc_pg::PgDriver {},
        "postgres://postgres:123456@localhost:5432/postgres",
    )?;
    //insert to log intercept before
    rb.intercepts.insert(0, Arc::new(ReturningIdPlugin {}));
    // the example must provide its own table (build.rs only creates the sqlite ones)
    _ = rb
        .exec(
            "CREATE TABLE IF NOT EXISTS activity (id TEXT PRIMARY KEY, name TEXT, pc_link TEXT, h5_link TEXT, pc_banner_img TEXT, h5_banner_img TEXT, sort TEXT, status INT8, remark TEXT, create_time TEXT, version INT8, delete_flag INT8)",
            vec![],
        )
        .await?;
    // clean up a previous run, so the insert of id '2' works again
    _ = rb.exec("delete from activity where id = '2'", vec![]).await;
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
    Ok(())
}
