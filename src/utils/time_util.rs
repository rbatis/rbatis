use std::time::{Duration, SystemTime, Instant};

pub fn count_time_qps(tag: &str, total: u128, start: Instant) {
    print_qps(tag, total, start);
    print_each_time(tag, total, start);
}


///count qps
pub fn print_qps(tag: &str, total: u128, start: Instant) {
    let time = start.elapsed().as_nanos();
    println!("[count_qps] {} use qps: {} QPS/s", tag, (total * 1000000000 as u128 / time));
}

///计算每个操作耗时ns纳秒
pub fn print_each_time(tag: &str, total: u128, start: Instant) {
    let time = start.elapsed();
    println!("[count_each_time] {} use Time: {},each:{} ns/op", tag, duration_to_string(time), time.as_nanos() / total as u128);
}

/// count wait time
pub fn print_time(tag: &str, start: Instant) {
    let time = start.elapsed();
    println!("[count_wait_time] {} use Time: {} ", tag, duration_to_string(time));
}

/// duration_to_string
fn duration_to_string(wait: Duration) -> String {
    if wait.gt(&Duration::from_millis(1)) {
        return format!("{}ms", wait.as_millis());
    } else if wait.gt(&Duration::from_secs(1)) {
        return format!("{}s", wait.as_secs() as u128);
    } else {
        return format!("{}ns", wait.as_nanos());
    }
}