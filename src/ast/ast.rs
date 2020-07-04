use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

/// Abstract syntax tree node
pub trait RbatisAST: Send + Sync + Clone {
    /// env: &mut Value,因为bind node 会绑定变量，env必须为可修改的值 arg_result: 执行后 提交到 驱动的参数
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_result: &mut Vec<Value>) -> Result<String, rbatis_core::Error>;
}