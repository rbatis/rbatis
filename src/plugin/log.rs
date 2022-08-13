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
        return !self.get_level_filter().eq(&LevelFilter::Off);
    }
    fn do_log(&self, level: LevelFilter, data: &str) {
        if self.get_level_filter() < level {
            return;
        }
        match level {
            LevelFilter::Error => {
                error!("{}",data)
            }
            LevelFilter::Warn => {
                warn!("{}",data)
            }
            LevelFilter::Info => {
                info!("{}",data)
            }
            LevelFilter::Debug => {
                debug!("{}",data)
            }
            LevelFilter::Trace => {
                trace!("{}",data)
            }
            LevelFilter::Off => {}
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
