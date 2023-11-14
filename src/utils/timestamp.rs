use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get system time");
    since_epoch.as_millis() as i64
}
