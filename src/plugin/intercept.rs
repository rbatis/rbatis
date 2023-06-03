use crate::decode::is_debug_mode;
use crate::executor::Executor;
use crate::Error;
use futures::future::Either;
use rbdc::db::ExecResult;
use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use log::{Level, LevelFilter, log};

/// sql intercept
pub trait SqlIntercept: Send + Sync {
    fn name(&self)->&str{
        std::any::type_name::<Self>()
    }
    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    /// result = None(before post(sql,arg)->Result)
    /// result = Some(after post(sql,arg)->Result)
    fn do_intercept(
        &self,
        task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>,
    ) -> Result<(), Error>;
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

/// LogInterceptor
#[derive(Debug,Clone)]
pub struct LogInterceptor {
    pub level_filter: LevelFilter,
}

impl LogInterceptor {
    pub fn new() -> Self {
        Self {
            level_filter: LevelFilter::Info,
        }
    }
    pub fn to_level(&self) -> Option<Level> {
        match self.level_filter {
            LevelFilter::Off => { None }
            LevelFilter::Error => { Some(Level::Error) }
            LevelFilter::Warn => { Some(Level::Warn) }
            LevelFilter::Info => { Some(Level::Info) }
            LevelFilter::Debug => { Some(Level::Debug) }
            LevelFilter::Trace => { Some(Level::Trace) }
        }
    }
}

impl SqlIntercept for LogInterceptor {
    fn do_intercept(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: Option<Result<Either<&ExecResult, &Vec<Value>>, &Error>>,
    ) -> Result<(), Error> {
        if self.level_filter == LevelFilter::Off {
            return Ok(());
        }
        let level = self.to_level().unwrap();
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
                            log!(level,"[rbatis] [{}] {}  <= rows_affected={}",task_id, op, result);
                        }
                        Either::Right(data) => {
                            if is_debug_mode() {
                                log!(level,"[rbatis] [{}] {} <= len={},rows={}",
                                        task_id,
                                        op,
                                        data.len(),
                                        RbsValueMutDisplay { inner: data }
                                );
                            } else {
                                log!(level,"[rbatis] [{}] {} <= len={}", task_id, op, data.len());
                            }
                        }
                    }
                }
                Err(e) => {
                    log!(level,"[rbatis] [{}] exec  <= {}", task_id, e);
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
            log!(level,"[rbatis] [{}] {} => `{}` {}",task_id,op,&sql,RbsValueMutDisplay { inner: args });
        }
        Ok(())
    }
}
