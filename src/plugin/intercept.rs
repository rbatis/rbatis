use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use futures::future::Either;
use log::LevelFilter;
use rbdc::db::ExecResult;
use crate::Error;
use rbs::Value;
use crate::decode::is_debug_mode;
use crate::executor::Executor;

/// sql intercept
pub trait SqlIntercept: Send + Sync {
    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    /// result = None(post sql,arg)
    /// result = Some(recv query result)
    fn do_intercept(
        &self,
        task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>,
    ) -> Result<(), Error>;
}

/// Prevent full table updates and deletions
#[derive(Debug)]
pub struct BlockAttackDeleteInterceptor {}

impl SqlIntercept for BlockAttackDeleteInterceptor {
    fn do_intercept(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>,
    ) -> Result<(), Error> {
        if _result.is_some(){
            return Ok(());
        }
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
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>,
    ) -> Result<(), Error> {
        if _result.is_some(){
            return Ok(());
        }
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


/// LogInterceptor
#[derive(Debug)]
pub struct LogInterceptor {}

impl SqlIntercept for LogInterceptor {
    fn do_intercept(&self, task_id: i64, rb: &dyn Executor, sql: &mut String, args: &mut Vec<Value>, result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>) -> Result<(), Error> {
        if !rb.rbatis_ref().log_plugin.is_enable() {
            return Ok(());
        }
        struct RbsValueMutDisplay<'a> {
            inner: &'a Vec<Value>,
        }
        impl<'a> Display for RbsValueMutDisplay<'a> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str("[")?;
                let mut idx = 0;
                for x in self.inner.deref() {
                    std::fmt::Display::fmt(x, f)?;
                    if (idx + 1) < self.inner.len() {
                        f.write_str(",")?;
                    }
                    idx += 1;
                }
                f.write_str("]")?;
                Ok(())
            }
        }
        if let Some(result) = result {
            //recv sql/args
            match result {
                Ok(result) => {
                    let op;
                    if sql.trim_start().starts_with("select") {
                        op = "query";
                    } else {
                        op = "exec ";
                    }
                    match result {
                        Either::Left(result) => {
                            rb.rbatis_ref().log_plugin.do_log(
                                LevelFilter::Info,
                                format!(
                                    "[rbatis] [{}] {}  <= rows_affected={}",
                                    task_id, op, result
                                ),
                            );
                        }
                        Either::Right(data) => {
                            if is_debug_mode() {
                                rb.rbatis_ref().log_plugin.do_log(
                                    LevelFilter::Info,
                                    format!("[rbatis] [{}] {} <= len={},rows={}", task_id, op, data.len(), RbsValueMutDisplay {
                                        inner: data
                                    }),
                                );
                            } else {
                                rb.rbatis_ref().log_plugin.do_log(
                                    LevelFilter::Info,
                                    format!("[rbatis] [{}] {} <= len={}", task_id, op, data.len()),
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    rb.rbatis_ref().log_plugin.do_log(
                        LevelFilter::Error,
                        format!("[rbatis] [{}] exec  <= {}", task_id, e),
                    );
                }
            }
        } else {
            //send sql/args
            let op;
            if sql.trim_start().starts_with("select") {
                op = "query";
            } else {
                op = "exec ";
            }
            rb.rbatis_ref().log_plugin.do_log(
                LevelFilter::Info,
                format!(
                    "[rbatis] [{}] {} => `{}` {}",
                    task_id,
                    op,
                    &sql,
                    RbsValueMutDisplay {
                        inner: args
                    }
                ),
            );
        }
        Ok(())
    }
}
