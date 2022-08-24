use std::collections::HashMap;
use std::ops::Deref;
use log::{debug, error, info, trace, warn, LevelFilter};
use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicI8, Ordering};

/// log plugin
pub trait LogPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn get_level_filter(&self) -> LevelFilter;
    fn set_level_filter(&self, level: LevelFilter);
    fn get_level(&self, level: LevelFilter) -> LevelFilter;
    fn set_level(&self, from: LevelFilter, to: LevelFilter);
    fn is_enable(&self) -> bool {
        return !self.get_level_filter().eq(&LevelFilter::Off);
    }
    fn do_log(&self, level: LevelFilter, data: &str) {
        if self.get_level_filter() < level {
            return;
        }
        let level = self.get_level(level);
        match level {
            LevelFilter::Error => {
                error!("{}", data)
            }
            LevelFilter::Warn => {
                warn!("{}", data)
            }
            LevelFilter::Info => {
                info!("{}", data)
            }
            LevelFilter::Debug => {
                debug!("{}", data)
            }
            LevelFilter::Trace => {
                trace!("{}", data)
            }
            LevelFilter::Off => {}
        }
    }
}

#[derive(Debug)]
pub struct RbatisLogPlugin {
    pub level_filter: AtomicI8,
    pub error: AtomicI8,
    pub warn: AtomicI8,
    pub info: AtomicI8,
    pub debug: AtomicI8,
    pub trace: AtomicI8,
}


impl Default for RbatisLogPlugin {
    //default leve info
    fn default() -> Self {
        RbatisLogPlugin {
            level_filter: AtomicI8::new(3),//info
            error: AtomicI8::new(1),
            warn: AtomicI8::new(2),
            info: AtomicI8::new(3),
            debug: AtomicI8::new(4),
            trace: AtomicI8::new(5),
        }
    }
}

impl RbatisLogPlugin{
    fn i8_to_level(level:i8) -> LevelFilter{
        match level{
            0=>LevelFilter::Off,
            1=>LevelFilter::Error,
            2=>LevelFilter::Warn,
            3=>LevelFilter::Info,
            4=>LevelFilter::Debug,
            5=>LevelFilter::Trace,
            _ => LevelFilter::Off
        }
    }
}

impl LogPlugin for RbatisLogPlugin {
    fn get_level_filter(&self) -> LevelFilter {
        RbatisLogPlugin::i8_to_level(self.level_filter.load(Ordering::SeqCst))
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

    fn get_level(&self, level: LevelFilter) -> LevelFilter {
        match level {
            LevelFilter::Off => LevelFilter::Off,
            LevelFilter::Error => RbatisLogPlugin::i8_to_level(self.error.load(Ordering::SeqCst)),
            LevelFilter::Warn => RbatisLogPlugin::i8_to_level(self.warn.load(Ordering::SeqCst)),
            LevelFilter::Info => RbatisLogPlugin::i8_to_level(self.info.load(Ordering::SeqCst)),
            LevelFilter::Debug => RbatisLogPlugin::i8_to_level(self.debug.load(Ordering::SeqCst)),
            LevelFilter::Trace => RbatisLogPlugin::i8_to_level(self.trace.load(Ordering::SeqCst)),
        }
    }

    fn set_level(&self, from: LevelFilter, to: LevelFilter) {
        let i = match to {
            LevelFilter::Error => 1,
            LevelFilter::Warn => 2,
            LevelFilter::Info => 3,
            LevelFilter::Debug => 4,
            LevelFilter::Trace => 5,
            LevelFilter::Off => {
                return;
            }
        };
        match from {
            LevelFilter::Error => {
                self.error.store(i, Ordering::SeqCst);
            }
            LevelFilter::Warn => {
                self.warn.store(i, Ordering::SeqCst);
            }
            LevelFilter::Info => {
                self.info.store(i, Ordering::SeqCst);
            }
            LevelFilter::Debug => {
                self.debug.store(i, Ordering::SeqCst);
            }
            LevelFilter::Trace => {
                self.trace.store(i, Ordering::SeqCst);
            }
            LevelFilter::Off => {
                self.set_level_filter(LevelFilter::Off)
            }
        }
    }
}
