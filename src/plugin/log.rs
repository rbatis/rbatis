use log::{debug, error, info, LevelFilter, trace, warn};
use std::ops::Deref;
/// log plugin
pub trait LogPlugin: Send + Sync {
    fn is_enable(&self) -> bool;
    fn do_log(&self, data: &str);
    fn error(&self, data: &str);
    fn warn(&self, data: &str);
    fn info(&self, data: &str);
    fn debug(&self, data: &str);
    fn trace(&self, data: &str);
}

pub struct RbatisLog {
    pub filter: LevelFilter
}

impl Default for RbatisLog {
    fn default() -> Self {
        Self {
            filter: log::LevelFilter::Info
        }
    }
}

impl LogPlugin for RbatisLog {
    fn is_enable(&self) -> bool {
        return !self.filter.eq(&log::LevelFilter::Off);
    }

    fn do_log(&self, data: &str) {
        match self.filter {
            log::LevelFilter::Error => {
                self.error(data);
            }
            log::LevelFilter::Warn => {
                self.warn(data);
            }
            log::LevelFilter::Info => {
                self.info(data);
            }
            log::LevelFilter::Debug => {
                self.debug(data);
            }
            log::LevelFilter::Trace => {
                self.trace(data);
            }
            _ => {}
        }
    }

    fn error(&self, data: &str) {
        error!("{}", data);
    }

    fn warn(&self, data: &str) {
        warn!("{}", data);
    }

    fn info(&self, data: &str) {
        info!("{}", data);
    }

    fn debug(&self, data: &str) {
        debug!("{}", data);
    }

    fn trace(&self, data: &str) {
        trace!("{}", data);
    }
}