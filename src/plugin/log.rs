use std::ops::Deref;

use log::{debug, error, info, LevelFilter, trace, warn};
use std::fmt::{Display, Debug};

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
            _ => {}
        }
    }
    fn error(&self, data: &str);
    fn warn(&self, data: &str);
    fn info(&self, data: &str);
    fn debug(&self, data: &str);
    fn trace(&self, data: &str);
}

#[derive(Debug)]
pub struct RbatisLog {
    pub level_filter: LevelFilter
}

impl Default for RbatisLog {
    fn default() -> Self {
        Self {
            level_filter: log::LevelFilter::Info
        }
    }
}

impl LogPlugin for RbatisLog {
    fn get_level_filter(&self) -> &LevelFilter {
        &self.level_filter
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