use std::time::SystemTime;

pub struct Bencher {
    total: u64,
    now: SystemTime,
}

impl Bencher {
    pub fn new(total: u64) -> Self {
        return Self {
            total,
            now: SystemTime::now(),
        };
    }

    pub fn iter<T, F>(&mut self, func: F) where F: Fn() {
        let mut current = 0;
        self.now = SystemTime::now();
        loop {
            func();
            if current == self.total - 1 {
                let end = SystemTime::now();
                Self::time(self.total, self.now, end);
                Self::qps(self.total, self.now, end);
                break;
            } else {
                current = current + 1;
            }
        }
    }

    pub fn iter_ref<T, F>(&mut self, arg: &T, func: F) where F: Fn(&T) {
        let mut current = 0;
        self.now = SystemTime::now();
        loop {
            func(arg);
            if current == self.total - 1 {
                let end = SystemTime::now();
                Self::time(self.total, self.now, end);
                Self::qps(self.total, self.now, end);
                break;
            } else {
                current = current + 1;
            }
        }
    }

    pub fn iter_mut<T, F>(&mut self, arg: &mut T, func: F) where F: Fn(&mut T) {
        let mut current = 0;
        self.now = SystemTime::now();
        loop {
            func(arg);
            if current == self.total - 1 {
                let end = SystemTime::now();
                Self::time(self.total, self.now, end);
                Self::qps(self.total, self.now, end);
                break;
            } else {
                current = current + 1;
            }
        }
    }

    pub fn qps(total: u64, start: SystemTime, end: SystemTime) {
        let time = end.duration_since(start).unwrap();
        println!("use TPS: {} QPS/s", (total as u128 * 1000000000 as u128 / time.as_nanos() as u128));
    }

    //计算每个操作耗时nano纳秒
    pub fn time(total: u64, start: SystemTime, end: SystemTime) {
        let t = end.duration_since(start).unwrap();
        println!("use Time: {:?} ,each:{} ns/op", &t, t.as_nanos() / (total as u128));
    }

    pub fn cost(method: &str, start: SystemTime) {
        println!("cost_{}:{:?}", method, SystemTime::now().duration_since(start).unwrap());
    }
}


