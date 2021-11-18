use std::ops::Deref;

use log::{debug, error, info, trace, warn, LevelFilter};
use std::fmt::{Debug, Display};

/// log plugin
pub trait LogPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn get_level_filter(&self) -> &LevelFilter;
    fn is_enable(&self) -> bool {
        return !self.get_level_filter().eq(&log::LevelFilter::Off);
    }
    fn do_log(&self, id: i64, data: &str) {
        match self.get_level_filter() {
            log::LevelFilter::Error => {
                self.error(id, data);
            }
            log::LevelFilter::Warn => {
                self.warn(id, data);
            }
            log::LevelFilter::Info => {
                self.info(id, data);
            }
            log::LevelFilter::Debug => {
                self.debug(id, data);
            }
            log::LevelFilter::Trace => {
                self.trace(id, data);
            }
            log::LevelFilter::Off => {}
        }
    }

    fn error(&self, id: i64, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Error) {
            error!("[rbatis] [{}] {}", id ,data);
        }
    }

    fn warn(&self, id: i64, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Warn) {
            warn!("[rbatis] [{}] {}", id,data);
        }
    }

    fn info(&self, id: i64, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Info) {
            info!("[rbatis] [{}] {}", id,data);
        }
    }

    fn debug(&self, id: i64, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Debug) {
            debug!("[rbatis] [{}] {}", id,data);
        }
    }

    fn trace(&self, id: i64, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Trace) {
            trace!("[rbatis] [{}] {}", id,data);
        }
    }
}

#[derive(Debug)]
pub struct RbatisLogPlugin {
    pub level_filter: LevelFilter,
}

impl Default for RbatisLogPlugin {
    fn default() -> Self {
        Self {
            level_filter: log::LevelFilter::Info,
        }
    }
}

impl LogPlugin for RbatisLogPlugin {
    fn get_level_filter(&self) -> &LevelFilter {
        &self.level_filter
    }
}
