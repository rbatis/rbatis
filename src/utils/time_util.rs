extern crate chrono;

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;

use self::chrono::DateTime;

pub fn count_time_tps(total: i32, start: SystemTime) {
    count_tps(total, start);
    count_time(total, start);
}

#[test]
fn test_count() {
    let sys_time = SystemTime::now();
    count_time(1,sys_time);
}



pub fn count_tps(total: i32, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("use TPS: {} TPS/s", total as f64 / time.as_secs()  as f64);
}

//计算每个操作耗时nano纳秒
pub fn count_time(total: i32, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("use Time: {} s,each:{} nano/op", time.as_secs(), time.as_nanos() / total as u128);
}