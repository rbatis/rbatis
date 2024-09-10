use serde::ser::SerializeStruct;
use serde::{Deserializer, Serializer};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

///Snowflakes algorithm
#[derive(Debug)]
pub struct Snowflake {
    pub epoch: u64,
    pub worker_id: u64,
    pub sequence: AtomicU64,
    pub last_timestamp: AtomicU64,
}

impl serde::Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Snowflake", 5)?;
        s.serialize_field("epoch", &self.epoch)?;
        s.serialize_field("worker_id", &self.worker_id)?;
        s.serialize_field(
            "last_timestamp",
            &self.last_timestamp.load(Ordering::Relaxed),
        )?;
        s.serialize_field("sequence", &self.sequence.load(Ordering::Relaxed))?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct Snowflake {
            pub epoch: u64,
            pub worker_id: u64,
            pub last_timestamp: u64,
            pub sequence: u64,
        }
        let proxy = Snowflake::deserialize(deserializer)?;
        Ok(self::Snowflake {
            epoch: proxy.epoch,
            worker_id: proxy.worker_id,
            last_timestamp: AtomicU64::new(proxy.last_timestamp),
            sequence: AtomicU64::new(proxy.sequence),
        })
    }
}

impl Clone for Snowflake {
    fn clone(&self) -> Self {
        Self {
            epoch: self.epoch,
            worker_id: self.worker_id,
            last_timestamp: AtomicU64::new(self.last_timestamp.load(Ordering::Relaxed)),
            sequence: AtomicU64::new(self.sequence.load(Ordering::Relaxed)),
        }
    }
}

impl Snowflake {
    pub const fn default() -> Snowflake {
        Snowflake {
            epoch: 1_564_790_400_000,
            worker_id: 1,
            last_timestamp: AtomicU64::new(0 as u64),
            sequence: AtomicU64::new(0),
        }
    }

    pub const fn new(epoch: u64, worker_id: u64, last_timestamp: u64) -> Snowflake {
        Snowflake {
            epoch: epoch,
            worker_id: worker_id,
            last_timestamp: AtomicU64::new(last_timestamp),
            sequence: AtomicU64::new(0),
        }
    }

    pub fn set_epoch(&mut self, epoch: u64) -> &mut Self {
        self.epoch = epoch;
        self
    }

    pub fn set_worker_id(&mut self, worker_id: u64) -> &mut Self {
        self.worker_id = worker_id;
        self
    }

    pub fn set_datacenter_id(&mut self, last_timestamp: u64) -> &mut Self {
        self.last_timestamp = AtomicU64::new(last_timestamp);
        self
    }

    pub fn generate(&self) -> u64 {
        let mut now = self.get_timestamp();
        loop {
            let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);
            // If the current timestamp is smaller than the last recorded timestamp,
            // update the timestamp to the last recorded timestamp to prevent non-monotonic IDs.
            if now <= last_timestamp {
                now = last_timestamp;
            }
            // Compare and swap the last recorded timestamp with the current timestamp.
            // If the comparison succeeds, break the loop.
            if self
                .last_timestamp
                .compare_exchange(
                    last_timestamp,
                    now,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_ok()
            {
                break;
            }
        }
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        // Shift and combine the components to generate the final ID.
        let timestamp_shifted = now << 22;
        let worker_id_shifted = self.worker_id << 12;
        let id = timestamp_shifted + worker_id_shifted + sequence;
        id
    }

    fn get_timestamp(&self) -> u64 {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get system time");
        since_epoch.as_millis() as u64 - self.epoch
    }
}

pub static SNOWFLAKE: Snowflake = Snowflake::default();

///gen new snowflake_id
pub fn new_snowflake_id() -> i64 {
    SNOWFLAKE.generate() as i64
}

#[cfg(test)]
mod test {
    use crate::snowflake::{new_snowflake_id, Snowflake};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread::sleep;
    use std::time::Duration;
    use dark_std::sync::WaitGroup;

    #[test]
    fn test_gen() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
        sleep(Duration::from_secs(1));
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }

    #[test]
    fn test_ser_de() {
        let s = Snowflake::default();
        s.generate();
        let data = serde_json::to_string(&s).unwrap();
        println!("source:{}", serde_json::to_string(&s).unwrap());
        let r: Snowflake = serde_json::from_str(&data).unwrap();
        println!("new:{}", serde_json::to_string(&r).unwrap());
    }

    #[test]
    fn test_race() {
        let size = 100000;
        let mut v1: Vec<i64> = Vec::with_capacity(size);
        let mut v2: Vec<i64> = Vec::with_capacity(size);
        let mut v3: Vec<i64> = Vec::with_capacity(size);
        let mut v4: Vec<i64> = Vec::with_capacity(size);
        println!(
            "v1 len:{},v2 len:{},v3 len:{},v4 len:{}",
            v1.len(),
            v2.len(),
            v3.len(),
            v4.len()
        );
        let wg = WaitGroup::new();
        std::thread::scope(|s| {
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v1.push(new_snowflake_id());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v2.push(new_snowflake_id());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v3.push(new_snowflake_id());
                }
                drop(wg1);
            });
            s.spawn(|| {
                let wg1 = wg.clone();
                for _ in 0..size {
                    v4.push(new_snowflake_id());
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
        let mut snowflake = Snowflake::default();
        snowflake.generate();
        let initial_timestamp = snowflake.last_timestamp.load(Ordering::Relaxed);
        let initial_id = snowflake.generate();
        println!("initial_id={}", initial_id);
        snowflake.last_timestamp = AtomicU64::new(initial_timestamp - 1224655892);
        let new_id = snowflake.generate();
        println!("new_id____={}", new_id);
        assert!(new_id > initial_id);
    }
}
