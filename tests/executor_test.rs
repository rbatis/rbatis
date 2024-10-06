use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbdc::{Error};
use rbdc::rt::tokio;
use rbs::{to_value, Value};
use serde::{Deserialize, Serialize};
use rbatis_macro_driver::html_sql;
use rbexec::{crud, Executor};

pub struct TestExecutor {}

impl Executor for TestExecutor {
    fn exec(&self, _sql: &str, _args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        Box::pin(async {
            Ok(ExecResult {
                rows_affected: 1,
                last_insert_id: Default::default(),
            })
        })
    }

    fn query(&self, _sql: &str, _args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        Box::pin(async {
            Ok(to_value!([TestTable{
              id: Some(111.to_string())
            }]))
        })
    }

    fn driver_type(&self) -> Result<&str, Error> {
        Ok("sqlite")
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TestTable {
    pub id: Option<String>,
}
crud!(rbexec, TestTable{});

#[html_sql(
r#"<select id="select_by_id">
        `select * from activity`
        <where>
         <if test="id != null">
              ` and id = #{id}`
         </if>
        </where>
  </select>"#
)]
async fn select_by_id(
    rb: &dyn Executor,
    id: &str,
) -> rbatis::Result<Vec<TestTable>> {
    impled!()
}

#[tokio::test]
async fn test_crud() {
    let executor = TestExecutor {};
    let table = TestTable {
        id: Some(1.to_string()),
    };
    let d = TestTable::insert(&executor, &table).await.unwrap();
    println!("{}", d);
}


#[tokio::test]
async fn test_html() {
    let executor = TestExecutor {};
    let d = select_by_id(&executor, "111").await.unwrap();
    println!("{:?}", d);
}