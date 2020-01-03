extern crate chrono;

use chrono::Local;
use self::chrono::DateTime;


pub fn count_time_tps(total: i32, start: DateTime<Local>){
    count_tps(total,start);
    count_time(total,start);
}


pub fn count_tps(total: i32, start: DateTime<Local>) {
    let mut time = (Local::now().timestamp_millis() - start.timestamp_millis()) as f64;
    time = time / 1000.0;
    println!("use TPS: {} TPS/s", total as f64 / time);
}

//计算每个操作耗时nano纳秒
pub fn count_time(total: i32, start: DateTime<Local>) {
    let mut time = (Local::now().timestamp_millis() - start.timestamp_millis()) as f64;
    time = time / 1000.0;
    println!("use Time: {} s,each:{} nano/op", time, time * 1000000000.0 / (total as f64));
}


