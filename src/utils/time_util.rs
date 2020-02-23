use std::time::{SystemTime};

pub fn count_time_tps(total: u128, start: SystemTime) {
    count_tps(total, start);
    count_time(total, start);
}

#[test]
fn test_count() {
    let sys_time = SystemTime::now();
    count_time(1,sys_time);
}



pub fn count_tps(total: u128, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("use TPS: {} QPS/s", (total*1000000000 as u128  / time.as_nanos() as u128));
}

//计算每个操作耗时nano纳秒
pub fn count_time(total: u128, start: SystemTime) {
    let mut time = SystemTime::now().duration_since(start).unwrap();
    println!("use Time: {} s,each:{} nano/op", time.as_secs(), time.as_nanos() / total as u128);
}