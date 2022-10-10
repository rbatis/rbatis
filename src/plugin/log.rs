use log::{debug, error, info, trace, warn, Level, LevelFilter};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicI8, Ordering};

/// log plugin
pub trait LogPlugin: Send + Sync {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn get_level_filter(&self) -> LevelFilter;
    /// filter rbatis log level
    fn set_level_filter(&self, level: LevelFilter);
    fn get_level(&self, level: Level) -> Level;
    /// change rbatis level print
    fn set_level(&self, from: Level, to: Level);
    fn is_enable(&self) -> bool {
        return !self.get_level_filter().eq(&LevelFilter::Off);
    }
    fn do_log(&self, level: LevelFilter, data: &str) {
        if self.get_level_filter().eq(&LevelFilter::Off) {
            return;
        }
        let level = self.get_level({
            match level {
                LevelFilter::Off => {
                    return;
                }
                LevelFilter::Error => Level::Error,
                LevelFilter::Warn => Level::Warn,
                LevelFilter::Info => Level::Info,
                LevelFilter::Debug => Level::Debug,
                LevelFilter::Trace => Level::Trace,
            }
        });
        match level {
            Level::Error => {
                error!("{}", data)
            }
            Level::Warn => {
                warn!("{}", data)
            }
            Level::Info => {
                info!("{}", data)
            }
            Level::Debug => {
                debug!("{}", data)
            }
            Level::Trace => {
                trace!("{}", data)
            }
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
            level_filter: AtomicI8::new(3), //info
            error: AtomicI8::new(1),
            warn: AtomicI8::new(2),
            info: AtomicI8::new(3),
            debug: AtomicI8::new(4),
            trace: AtomicI8::new(5),
        }
    }
}

impl RbatisLogPlugin {
    fn i8_to_level(level: i8) -> Level {
        match level {
            1 => Level::Error,
            2 => Level::Warn,
            3 => Level::Info,
            4 => Level::Debug,
            5 => Level::Trace,
            _ => {
                panic!("unknown level:{}", level)
            }
        }
    }
    fn i8_to_level_filter(level: i8) -> LevelFilter {
        match level {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            5 => LevelFilter::Trace,
            _ => LevelFilter::Off,
        }
    }
    fn level_filter_to_i8(to: LevelFilter) -> i8 {
        match to {
            LevelFilter::Off => 0,
            LevelFilter::Error => 1,
            LevelFilter::Warn => 2,
            LevelFilter::Info => 3,
            LevelFilter::Debug => 4,
            LevelFilter::Trace => 5,
        }
    }
    fn level_to_i8(to: Level) -> i8 {
        match to {
            Level::Error => 1,
            Level::Warn => 2,
            Level::Info => 3,
            Level::Debug => 4,
            Level::Trace => 5,
        }
    }
}

impl LogPlugin for RbatisLogPlugin {
    fn get_level_filter(&self) -> LevelFilter {
        RbatisLogPlugin::i8_to_level_filter(self.level_filter.load(Ordering::SeqCst))
    }

    fn set_level_filter(&self, level: LevelFilter) {
        self.level_filter
            .store(RbatisLogPlugin::level_filter_to_i8(level), Ordering::SeqCst);
    }

    fn get_level(&self, level: Level) -> Level {
        match level {
            Level::Error => RbatisLogPlugin::i8_to_level(self.error.load(Ordering::SeqCst)),
            Level::Warn => RbatisLogPlugin::i8_to_level(self.warn.load(Ordering::SeqCst)),
            Level::Info => RbatisLogPlugin::i8_to_level(self.info.load(Ordering::SeqCst)),
            Level::Debug => RbatisLogPlugin::i8_to_level(self.debug.load(Ordering::SeqCst)),
            Level::Trace => RbatisLogPlugin::i8_to_level(self.trace.load(Ordering::SeqCst)),
        }
    }

    fn set_level(&self, from: Level, to: Level) {
        let i = RbatisLogPlugin::level_to_i8(to);
        match from {
            Level::Error => {
                self.error.store(i, Ordering::SeqCst);
            }
            Level::Warn => {
                self.warn.store(i, Ordering::SeqCst);
            }
            Level::Info => {
                self.info.store(i, Ordering::SeqCst);
            }
            Level::Debug => {
                self.debug.store(i, Ordering::SeqCst);
            }
            Level::Trace => {
                self.trace.store(i, Ordering::SeqCst);
            }
        }
    }
}
