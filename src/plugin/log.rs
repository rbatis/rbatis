use std::ops::Deref;

use log::{debug, error, info, trace, warn, LevelFilter};
use std::fmt::{Debug, Display};
use std::sync::atomic::{AtomicI8, Ordering};

/// log plugin
pub trait LogPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn get_level_filter(&self) -> LevelFilter;
    fn set_level_filter(&self, level: LevelFilter);
    fn is_enable(&self) -> bool {
        return !self.get_level_filter().eq(&log::LevelFilter::Off);
    }
    fn do_log(&self, data: &str) {
        match self.get_level_filter() {
            log::LevelFilter::Error => {
                self.error(data);
            }
            log::LevelFilter::Warn => {
                self.warn( data);
            }
            log::LevelFilter::Info => {
                self.info( data);
            }
            log::LevelFilter::Debug => {
                self.debug( data);
            }
            log::LevelFilter::Trace => {
                self.trace(data);
            }
            log::LevelFilter::Off => {}
        }
    }

    fn error(&self,  data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Error) {
            error!("{}" ,data);
        }
    }

    fn warn(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Warn) {
            warn!(" {}", data);
        }
    }

    fn info(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Info) {
            info!("{}", data);
        }
    }

    fn debug(&self,  data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Debug) {
            debug!("{}",data);
        }
    }

    fn trace(&self, data: &str) {
        let filter = self.get_level_filter();
        if filter.eq(&LevelFilter::Off) {
            return;
        }
        if filter.ge(&LevelFilter::Trace) {
            trace!("{}", data);
        }
    }
}

#[derive(Debug)]
pub struct RbatisLogPlugin {
    pub level_filter: AtomicI8,
}

impl From<&RbatisLogPlugin> for LevelFilter {
    fn from(arg: &RbatisLogPlugin) -> Self {
        match arg.level_filter.load(Ordering::SeqCst) {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            5 => LevelFilter::Trace,
            _ => LevelFilter::Trace,
        }
    }
}

impl Default for RbatisLogPlugin {
    //default leve info
    fn default() -> Self {
        RbatisLogPlugin {
            level_filter: AtomicI8::new(3),
        }
    }
}

impl LogPlugin for RbatisLogPlugin {
    fn get_level_filter(&self) -> LevelFilter {
        self.into()
    }

    fn set_level_filter(&self, level: LevelFilter) {
        match level {
            LevelFilter::Off => {
                self.level_filter.store(0, Ordering::SeqCst);
            }
            LevelFilter::Error => {
                self.level_filter.store(1, Ordering::SeqCst);
            }
            LevelFilter::Warn => {
                self.level_filter.store(2, Ordering::SeqCst);
            }
            LevelFilter::Info => {
                self.level_filter.store(3, Ordering::SeqCst);
            }
            LevelFilter::Debug => {
                self.level_filter.store(4, Ordering::SeqCst);
            }
            LevelFilter::Trace => {
                self.level_filter.store(5, Ordering::SeqCst);
            }
        }
    }
}
