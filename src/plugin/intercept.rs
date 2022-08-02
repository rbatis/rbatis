use crate::core::convert::StmtConvert;
use crate::crud::CRUDTable;
use crate::rbatis::Rbatis;
use rbatis_core::Error;
use rbson::Bson;
use std::fmt::{Debug, Display};

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
        args: &mut Vec<rbson::Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), crate::core::Error>;
}

#[derive(Debug)]
pub struct RbatisLogFormatSqlIntercept {}

impl SqlIntercept for RbatisLogFormatSqlIntercept {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        let driver_type = rb.driver_type()?;
        let mut formated = format!("[format_sql]{}", sql);
        for index in 0..args.len() {
            let mut data = String::new();
            driver_type.stmt_convert(index, &mut data);
            formated =
                formated.replacen(&data, &format!("{}", args.get(index).unwrap()), 1);
        }
        rb.log_plugin.info(0, &formated);
        return Ok(());
    }
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackDeleteInterceptor {}

impl SqlIntercept for BlockAttackDeleteInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
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
        args: &mut Vec<Bson>,
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
