use crate::decode::is_debug_mode;
use crate::executor::Executor;
use crate::intercept::{Intercept, ResultType};
use crate::Error;
use async_trait::async_trait;
use log::{log, Level, LevelFilter};
use rbdc::db::ExecResult;
use rbs::Value;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

struct RbsValueDisplay<'a> {
    inner: &'a Vec<Value>,
}

impl<'a> RbsValueDisplay<'a> {
    pub fn new(v: &'a Vec<Value>) -> Self {
        Self { inner: v }
    }
}

impl<'a> Display for RbsValueDisplay<'a> {
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
#[derive(Debug)]
pub struct LogInterceptor {
    ///control log off,or change log level.
    /// 0=Off,
    /// 1=Error,
    /// 2=Warn,
    /// 3=Info,
    /// 4=Debug,
    /// 5=Trace
    pub level_filter: AtomicUsize,
}

impl Clone for LogInterceptor {
    fn clone(&self) -> Self {
        LogInterceptor::new(self.get_level_filter())
    }
}

impl LogInterceptor {
    pub fn new(level_filter: LevelFilter) -> Self {
        let s = Self {
            level_filter: AtomicUsize::new(0),
        };
        s.set_level_filter(level_filter);
        s
    }

    pub fn get_level_filter(&self) -> LevelFilter {
        match self.level_filter.load(Ordering::Relaxed) {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            5 => LevelFilter::Trace,
            _ => LevelFilter::Off,
        }
    }

    pub fn to_level(&self) -> Option<Level> {
        match self.get_level_filter() {
            LevelFilter::Off => None,
            LevelFilter::Error => Some(Level::Error),
            LevelFilter::Warn => Some(Level::Warn),
            LevelFilter::Info => Some(Level::Info),
            LevelFilter::Debug => Some(Level::Debug),
            LevelFilter::Trace => Some(Level::Trace),
        }
    }

    pub fn set_level_filter(&self, level_filter: LevelFilter) {
        match level_filter {
            LevelFilter::Off => self.level_filter.store(0, Ordering::SeqCst),
            LevelFilter::Error => self.level_filter.store(1, Ordering::SeqCst),
            LevelFilter::Warn => self.level_filter.store(2, Ordering::SeqCst),
            LevelFilter::Info => self.level_filter.store(3, Ordering::SeqCst),
            LevelFilter::Debug => self.level_filter.store(4, Ordering::SeqCst),
            LevelFilter::Trace => self.level_filter.store(5, Ordering::SeqCst),
        }
    }
}

#[async_trait]
impl Intercept for LogInterceptor {
    async fn before(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if self.get_level_filter() == LevelFilter::Off {
            return Ok(Some(true));
        }
        let level = self.to_level().unwrap_or(Level::Debug);
        //send sql/args
        log!(
            level,
            "[rb] [{}] => `{}` {}",
            task_id,
            &sql,
            RbsValueDisplay::new(args)
        );
        Ok(Some(true))
    }

    async fn after(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if self.get_level_filter() == LevelFilter::Off {
            return Ok(Some(true));
        }
        let level = self.to_level().unwrap_or_else(||Level::Debug);
        //ResultType
        match result {
            ResultType::Exec(result) => match result {
                Ok(result) => {
                    log!(level, "[rb] [{}] <= rows_affected={}", task_id, result);
                }
                Err(e) => {
                    log!(level, "[rb] [{}] <= {}", task_id, e);
                }
            },
            ResultType::Query(result) => match result {
                Ok(result) => {
                    if is_debug_mode() {
                        log!(
                            level,
                            "[rb] [{}] <= len={},rows={}",
                            task_id,
                            result.len(),
                            RbsValueDisplay { inner: result }
                        );
                    } else {
                        log!(level, "[rb] [{}] <= len={}", task_id, result.len());
                    }
                }
                Err(e) => {
                    log!(level, "[rb] [{}] <= {}", task_id, e);
                }
            },
        }
        Ok(Some(true))
    }
}
