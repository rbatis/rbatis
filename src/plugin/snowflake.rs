use std::hint::spin_loop;
use std::sync::atomic::{AtomicI64, AtomicU16, Ordering};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::ReentrantMutex;

///Snowflakes algorithm
#[derive(Debug)]
pub struct Snowflake {
    epoch: SystemTime,
    last_timestamp: AtomicI64, // Atomic to replace mutable i64
    pub machine_id: i32,
    pub node_id: i32,
    pub mode: i32, // mode [0=fast_generate,1=realtime_generate]
    idx: AtomicU16, // Atomic to replace mutable u16
    lock: ReentrantMutex<()>,
}

impl Clone for Snowflake {
    fn clone(&self) -> Self {
        Self {
            epoch: self.epoch,
            machine_id: self.machine_id,
            node_id: self.node_id,
            last_timestamp: AtomicI64::new(self.last_timestamp.load(Ordering::SeqCst)),
            idx: AtomicU16::new(self.idx.load(Ordering::SeqCst)),
            lock: ReentrantMutex::new(()),
            mode: self.mode,
        }
    }
}

impl Snowflake {
    pub fn new(machine_id: i32, node_id: i32,mode:i32) -> Snowflake {
        Self::with_epoch(machine_id, node_id, mode, UNIX_EPOCH)
    }

    pub fn with_epoch(machine_id: i32, node_id: i32, mode: i32, epoch: SystemTime) -> Snowflake {
        let last_time_millis = Self::get_time_millis(epoch);
        Snowflake {
            epoch,
            last_timestamp: AtomicI64::new(last_time_millis),
            machine_id,
            node_id,
            mode,
            idx: AtomicU16::new(0),
            lock: ReentrantMutex::new(()),
        }
    }
    pub fn default() -> Snowflake {
        Snowflake::new(1, 1,0)
    }

    #[inline]
    pub fn generate(&self) -> i64 {
        let g = self.lock.lock();
        let mut idx = self.idx.fetch_add(1, Ordering::SeqCst) % 4096;
        if idx == 0 {
            // use timestamp
            if self.mode == 1 {
                let mut now_millis = Self::get_time_millis(self.epoch);
                let last_time = self.last_timestamp.load(Ordering::SeqCst);
                if now_millis == last_time {
                    now_millis = Self::biding_time_conditions(last_time, self.epoch);
                }
                self.last_timestamp.store(now_millis, Ordering::SeqCst);
            } else {
                //mode = 0,this is fast mode
                self.last_timestamp.fetch_add(1, Ordering::SeqCst);
            }
            drop(g);
        }
        let last_time = self.last_timestamp.load(Ordering::SeqCst);
        last_time << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (idx as i64)
    }


    #[inline(always)]
    pub fn get_time_millis(epoch: SystemTime) -> i64 {
        SystemTime::now()
            .duration_since(epoch)
            .expect("Time went backward")
            .as_millis() as i64
    }

    #[inline(always)]
    fn biding_time_conditions(last_time_millis: i64, epoch: SystemTime) -> i64 {
        let mut latest_time_millis: i64;
        loop {
            latest_time_millis = Self::get_time_millis(epoch);
            if latest_time_millis > last_time_millis {
                return latest_time_millis;
            }
            spin_loop();
        }
    }
}

pub static SNOWFLAKE: LazyLock<Snowflake> = LazyLock::new(|| {
    Snowflake::new(1, 1,0)
});

///gen new snowflake_id
pub fn new_snowflake_id() -> i64 {
    SNOWFLAKE.generate() as i64
}

#[cfg(test)]
mod test {
    use crate::snowflake::{new_snowflake_id, Snowflake};
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;
    use dark_std::sync::WaitGroup;

    #[test]
    fn test_gen() {
        let id = Snowflake::new(1, 1,0);
        println!("{}", id.generate());
        sleep(Duration::from_secs(1));
        println!("{}", id.generate());
    }

    #[test]
    fn test_gen1() {
        let id = Snowflake::new(1, 1,1);
        println!("{}", id.generate());
        println!("{}", id.generate());
        sleep(Duration::from_secs(1));
        println!("{}", id.generate());
        println!("{}", id.generate());
    }

    #[test]
    fn test_race() {
        let id_generator_generator = Snowflake::new(1, 1,0);
        let size = 1000000;
        let mut v1: Vec<i64> = Vec::with_capacity(size);
        let mut v2: Vec<i64> = Vec::with_capacity(size);
        let mut v3: Vec<i64> = Vec::with_capacity(size);
        let mut v4: Vec<i64> = Vec::with_capacity(size);
        let wg = WaitGroup::new();
        std::thread::scope(|s| {
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v1.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v2.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v3.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v4.push(id_generator_generator.generate());
                }
                drop(wg1);
            });
        });

        wg.wait();

        println!(
            "v1 len:{},v2 len:{},v3 len:{},v4 len:{}",
            v1.len(),
            v2.len(),
            v3.len(),
            v4.len()
        );
        let mut all: Vec<i64> = Vec::with_capacity(size * 4);
        all.append(&mut v1);
        all.append(&mut v2);
        all.append(&mut v3);
        all.append(&mut v4);

        let mut id_map: HashMap<i64, i64> = HashMap::with_capacity(all.len());
        for id in all {
            id_map
                .entry(id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for (_, v) in id_map {
            assert_eq!(v <= 1, true);
        }
    }

    #[test]
    fn test_generate_no_clock_back() {
        let snowflake = Snowflake::default();
        let id1 = snowflake.generate();
        let id2 = snowflake.generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_clock_rollback() {
        let id_generator_generator = Snowflake::new(1, 1,0);
        let initial_id = id_generator_generator.generate();
        println!("initial_id={}", initial_id);

        let new_id = id_generator_generator.generate();
        println!("new_id____={}", new_id);
        assert!(new_id > initial_id);
    }
}
