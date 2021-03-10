use serde_json::Value;
use std::fmt::{Debug, Display};
use rbatis_core::Error;
use crate::core::convert::StmtConvert;
use crate::crud::CRUDTable;
use crate::rbatis::Rbatis;
use crate::DriverType;

/// sql intercept
pub trait SqlIntercept: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(
        &self,
        rb: &Rbatis,
        context_id: &str,
        sql: &mut String,
        args: &mut Vec<serde_json::Value>,
        is_prepared_sql: bool,
    ) -> Result<(), crate::core::Error>;
}

#[derive(Debug)]
pub struct RbatisLogFormatSqlIntercept {}

impl SqlIntercept for RbatisLogFormatSqlIntercept {
    fn do_intercept(&self, rb: &Rbatis, context_id: &str, sql: &mut String, args: &mut Vec<Value>, is_prepared_sql: bool) -> Result<(), Error> {
        let driver_type = rb.driver_type()?;
        match driver_type {
            DriverType::None => {}
            DriverType::Mysql | DriverType::Postgres | DriverType::Sqlite | DriverType::Mssql => {
                let mut formated = format!("[format_sql]{}",sql);
                for index in 0..args.len() {
                    formated = formated.replacen(&driver_type.stmt_convert(index), &format!("{}", args.get(index).unwrap()),1);
                }
                rb.log_plugin.info(context_id, &formated);
            }
        }
        return Ok(());
    }
}