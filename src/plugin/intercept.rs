use std::fmt::{Display, Debug};
use serde_json::Value;

use rbatis_core::Error;

use crate::crud::CRUDEnable;
use crate::rbatis::Rbatis;
use crate::core::convert::StmtConvert;

/// sql intercept
pub trait SqlIntercept: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<serde_json::Value>, is_prepared_sql: bool) -> Result<(), crate::core::Error>;
}
