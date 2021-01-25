use serde_json::Value;
use std::fmt::{Debug, Display};

use crate::core::convert::StmtConvert;
use rexpr::runtime::RExprRuntime;

/// Abstract syntax tree node
pub trait RbatisAST: Send + Sync + Debug {
    fn name() -> &'static str
    where
        Self: Sized;
    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_result: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error>;
}
