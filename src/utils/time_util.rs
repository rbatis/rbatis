use std::time::SystemTime;

pub fn count_time_tps(tag: &str, total: u128, start: SystemTime) {
    count_tps(tag, total, start);
    count_each_time(tag, total, start);
}


//count tps
pub fn count_tps(tag: &str, total: u128, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("[count_tps] {} use TPS: {} QPS/s", tag, (total * 1000000000 as u128 / time.as_nanos() as u128));
}

//计算每个操作耗时nano纳秒
pub fn count_each_time(tag: &str, total: u128, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("[count_each_time] {} use Time: {} nano,each:{} nano/op", tag, time.as_nanos(), time.as_nanos() / total as u128);
}

//count wait time
pub fn count_wait_time(tag: &str, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("[count_wait_time] {} use Time: {} nano", tag, time.as_nanos());
}