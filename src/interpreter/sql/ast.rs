use serde::export::fmt::Debug;
use serde_json::Value;

use crate::core::convert::StmtConvert;
use crate::interpreter::expr::runtime::ExprRuntime;

/// Abstract syntax tree node
pub trait RbatisAST: Send + Sync + Debug {
    fn name() -> &'static str where Self: Sized;
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_result: &mut Vec<Value>) -> Result<String, crate::core::Error>;
}