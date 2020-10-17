/// log plugin
pub trait LogPlugin: Send + Sync {
    fn error(&self, data: &str);
    fn warn(&self, data: &str);
    fn info(&self, data: &str);
    fn debug(&self, data: &str);
    fn trace(&self, data: &str);
}


use log::{debug, error, info, LevelFilter, trace, warn};
pub struct RbatisLog {}

impl LogPlugin for RbatisLog {
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