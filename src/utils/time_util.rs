use std::time::Instant;

pub fn count_time_qps(tag: &str, total: u128, start: Instant) {
    print_qps(tag, total, start);
    print_each_time(tag, total, start);
}

///count qps
pub fn print_qps(tag: &str, total: u128, start: Instant) {
    let time = start.elapsed().as_nanos();
    println!(
        "[count_qps] {} use qps: {} QPS/s",
        tag,
        (total * 1000000000 as u128 / time)
    );
}

///计算每个操作耗时ns纳秒
pub fn print_each_time(tag: &str, total: u128, start: Instant) {
    let time = start.elapsed();
    println!(
        "[count_each_time] {} use Time: {:?},each:{} ns/op",
        tag,
        time,
        time.as_nanos() / total as u128
    );
}