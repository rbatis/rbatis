use crate::utils::time_util::count_time;
use chrono::{Local, DateTime};

pub struct Bencher {
    total: i32,
    now: i64,
}

impl Bencher {
    pub fn new(total: i32) -> Self {
        return Self {
            total,
            now: 0,
        };
    }

    pub fn iter(&mut self, func: fn()) {
        let mut current = 0;
        self.now = Local::now().timestamp_millis();
        loop {
            func();
            if current == self.total - 1 {
                let end = Local::now().timestamp_millis();
                use_time(self.total, self.now, end);
                use_tps(self.total, self.now, end);
                break;
            } else {
                current = current + 1;
            }
        }
    }
}

fn use_tps(total: i32, start: i64, end: i64) {
    let mut time = (end - start) as f64;
    time = time / 1000.0;
    println!("use TPS: {} TPS/s", total as f64 / time);
}

//计算每个操作耗时nano纳秒
fn use_time(total: i32, start: i64, end: i64) {
    let mut time = (end - start) as f64;
    time = time / 1000.0;
    println!("use Time: {} s,each:{} nano/op", time, time * 1000000000.0 / (total as f64));
}
