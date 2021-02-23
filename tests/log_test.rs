#[cfg(test)]
mod test {
    use rbatis::plugin::log::{RbatisLogPlugin, LogPlugin};

    #[test]
    fn test_level() {
        let wg = fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
        let mut plugin = RbatisLogPlugin::default();
        plugin.debug("debug", "11");
        plugin.info("info", "11");
        plugin.warn("warn", "11");
        plugin.error("error", "11");

        plugin.level_filter = log::LevelFilter::Off;
        plugin.info("off", "11");

        wg.exit_and_wait();
    }
}