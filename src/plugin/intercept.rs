use crate::core::convert::StmtConvert;
use crate::rbatis::Rbatis;
use rbatis_core::Error;
use std::fmt::{Debug, Display};
use rbs::Value;

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
        sql: &mut String,
        args: &mut Vec<Value>,
        is_prepared_sql: bool,
    ) -> Result<(), Error>;
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackDeleteInterceptor {}

impl SqlIntercept for BlockAttackDeleteInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Value>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let sql = sql.trim();
        if sql.starts_with(crate::sql::TEMPLATE.delete_from.value)
            && !sql.contains(crate::sql::TEMPLATE.r#where.left_right_space)
        {
            return Err(Error::from(format!(
                "[rbatis][BlockAttackDeleteInterceptor] not allow attack sql:{}",
                sql
            )));
        }
        return Ok(());
    }
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackUpdateInterceptor {}

impl SqlIntercept for BlockAttackUpdateInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Value>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let sql = sql.trim();
        if sql.starts_with(crate::sql::TEMPLATE.update.value)
            && !sql.contains(crate::sql::TEMPLATE.r#where.left_right_space)
        {
            return Err(Error::from(format!(
                "[rbatis][BlockAttackUpdateInterceptor] not allow attack sql:{}",
                sql
            )));
        }
        return Ok(());
    }
}
