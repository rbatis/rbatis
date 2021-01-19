pub trait QPS {
    fn qps(&self, total: u64);
    fn time(&self, total: u64);
    fn cost(&self);
}

impl QPS for std::time::Instant {
    fn qps(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use QPS: {} QPS/s",
            (total as u128 * 1000000000 as u128 / time.as_nanos() as u128)
        );
    }

    fn time(&self, total: u64) {
        let time = self.elapsed();
        println!(
            "use Time: {:?} ,each:{} ns/op",
            &time,
            time.as_nanos() / (total as u128)
        );
    }

    fn cost(&self) {
        let time = self.elapsed();
        println!("cost:{:?}", time);
    }
}
