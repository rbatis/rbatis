use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbdc::Error;
use rbdc::rt::tokio;
use rbs::{to_value, Value};
use serde::{Deserialize, Serialize};
use rbexec::{crud, Executor};

pub struct TestExecutor{}
impl Executor for TestExecutor{
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        Box::pin(async{
            Ok(ExecResult{
                rows_affected: 1,
                last_insert_id: Default::default(),
            })
        })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        Box::pin(async{
            Ok(to_value!([1,2,3]))
        })
    }

    fn driver_type(&self) -> Result<&str, Error> {
        Ok("sqlite")
    }
}


#[derive(Serialize,Deserialize)]
pub struct TestTable{
    pub id:Option<String>
}

crud!(rbexec, TestTable{});
#[tokio::test]
async fn test_crud(){
    let executor = TestExecutor{};
    let table=TestTable{
        id: Some(1.to_string()),
    };
    let d= TestTable::insert(&executor,&table).await.unwrap();
    println!("{}",d);
}