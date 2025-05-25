use async_trait::async_trait;
use rbdc::db::ExecResult;
use rbs::Value;
use crate::Error;
use crate::executor::Executor;
use crate::intercept::{Intercept, ResultType};

#[derive(Debug)]
pub struct CheckIntercept {
    
}

impl CheckIntercept {
    pub fn new() -> CheckIntercept {
        CheckIntercept {}
    }
}

#[async_trait]
impl Intercept for CheckIntercept {
    async fn before(
        &self,
        _task_id: i64,
        _executor: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        //check in empty array
        if sql.contains(" in ()"){
            match result {
                ResultType::Exec(exec) => {
                    *exec = Ok(ExecResult{
                        rows_affected: 0,
                        last_insert_id: Default::default(),
                    });
                }
                ResultType::Query(query) => {
                    *query = Ok(vec![]);
                }
            }
            return Ok(None);
        }
        Ok(Some(true))
    }
}