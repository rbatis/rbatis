use log::{error, info, warn};
use rdbc::{Connection, Value};
use serde::de;

use crate::decode::rdbc_driver_decoder::decode_result_set;

/// TODO 添加 tokio异步支持
use tokio::task;
use crate::error::RbatisError;

async fn docs() -> Result<(), Box<dyn std::error::Error>> {
    let res = task::spawn_blocking(move || {
        //do some compute-heavy work or call synchronous code
        "done computing"
    }).await?;
    assert_eq!(res, "done computing");
    Ok(())
}

///查询，执行接口
pub trait AbstractSession {
    fn query_prepare<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, RbatisError> where T: de::DeserializeOwned;
    fn exec_prepare(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64,  RbatisError>;
    fn query<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, RbatisError> where T: de::DeserializeOwned;
    fn exec(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64,  RbatisError>;

    fn query_custom<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value], is_prepare: bool) -> Result<T, RbatisError> where T: de::DeserializeOwned;
    fn exec_custom(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value], is_prepare: bool) -> Result<u64,  RbatisError>;
}

///查询和执行，带有prepare的是已编译的sql。
impl AbstractSession for Box<dyn Connection> {
    fn query_prepare<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        return self.query_custom(enable_log, sql, arg_array, true);
    }

    fn exec_prepare(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64,  RbatisError> {
        return self.exec_custom(enable_log, sql, arg_array, true);
    }

    fn query<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[Value]) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        return self.query_custom(enable_log, sql, arg_array, false);
    }

    fn exec(&mut self, enable_log: bool, sql: &str, arg_array: &[Value]) -> Result<u64,  RbatisError> {
        return self.exec_custom(enable_log, sql, arg_array, false);
    }

    fn query_custom<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value], is_prepare: bool) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        let create_result;
        if is_prepare {
            create_result = self.prepare(sql);
        } else {
            create_result = self.create(sql);
        }
        if create_result.is_err() {
            return Result::Err(RbatisError::from("[rbatis] select fail:".to_string() + format!("{:?}", create_result.err().unwrap()).as_str()));
        }
        let mut create_statement = create_result.unwrap();
        let exec_result = create_statement.execute_query(&arg_array);
        if exec_result.is_err() {
            return Result::Err(RbatisError::from("[rbatis] select fail:".to_string() + format!("{:?}", exec_result.err().unwrap()).as_str()));
        }
        let (result, decoded_num) = decode_result_set(exec_result.unwrap().as_mut());
        if enable_log {
            info!(" Total: <==  {}", decoded_num.to_string().as_str());
        }
        return result;
    }

    fn exec_custom(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value], is_prepare: bool) -> Result<u64,  RbatisError> {
        let create_result;
        if is_prepare {
            create_result = self.prepare(sql);
        } else {
            create_result = self.create(sql);
        }
        if create_result.is_err() {
            return Result::Err(RbatisError::from("[rbatis] exec fail:".to_string() + format!("{:?}", create_result.err().unwrap()).as_str()));
        }
        let exec_result = create_result.unwrap().execute_update(&arg_array);
        if exec_result.is_err() {
            return Result::Err(RbatisError::from("[rbatis] exec fail:".to_string() + format!("{:?}", exec_result.err().unwrap()).as_str()));
        }
        let affected_rows = exec_result.unwrap();
        if enable_log {
            info!(" Affected: <== {}", affected_rows.to_string().as_str());
        }
        return Result::Ok(affected_rows);
    }
}