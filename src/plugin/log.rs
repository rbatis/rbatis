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
    fn do_log(&self, data: &str) {
        match self.get_level_filter() {
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
            log::LevelFilter::Off => {}
        }
    }

    fn error(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Error) {
            error!("[rbatis] [] {}", data);
        }
    }

    fn warn(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Warn) {
            warn!("[rbatis] [] {}", data);
        }
    }

    fn info(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Info) {
            info!("[rbatis] [] {}", data);
        }
    }

    fn debug(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Debug) {
            debug!("[rbatis] [] {}", data);
        }
    }

    fn trace(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Trace) {
            trace!("[rbatis] [] {}", data);
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
