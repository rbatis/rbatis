use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use chrono::Utc;
use once_cell::sync::Lazy;
use serde::ser::SerializeStruct;
use serde::{Deserializer, Serializer};

///Snowflakes algorithm
#[derive(Debug)]
pub struct Snowflake {
    pub epoch: i64,
    pub worker_id: i64,
    pub datacenter_id: i64,
    pub sequence: AtomicI64,
    pub time: AtomicI64,
}

impl serde::Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Snowflake", 5)?;
        s.serialize_field("epoch", &self.epoch)?;
        s.serialize_field("worker_id", &self.worker_id)?;
        s.serialize_field("datacenter_id", &self.datacenter_id)?;
        s.serialize_field("sequence", &self.sequence.load(Ordering::Relaxed))?;
        s.serialize_field("time", &self.time.load(Ordering::Relaxed))?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for Snowflake {
    fn deserialize<D>(mut deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct Snowflake {
            pub epoch: i64,
            pub worker_id: i64,
            pub datacenter_id: i64,
            pub sequence: i64,
            pub time: i64,
        }
        let proxy = Snowflake::deserialize(deserializer)?;
        Ok(self::Snowflake {
            epoch: proxy.epoch,
            worker_id: proxy.worker_id,
            datacenter_id: proxy.datacenter_id,
            sequence: AtomicI64::from(proxy.sequence),
            time: AtomicI64::from(proxy.time),
        })
    }
}

impl Clone for Snowflake {
    fn clone(&self) -> Self {
        let sequence = self.sequence.load(Ordering::Relaxed);
        let time = self.time.load(Ordering::Relaxed);
        Self {
            epoch: self.epoch.clone(),
            worker_id: self.worker_id.clone(),
            datacenter_id: self.datacenter_id.clone(),
            sequence: AtomicI64::new(sequence),
            time: AtomicI64::new(time),
        }
    }
}

impl Snowflake {
    pub fn default() -> Snowflake {
        Snowflake {
            epoch: 1_564_790_400_000,
            worker_id: 1,
            datacenter_id: 1,
            sequence: AtomicI64::new(0),
            time: AtomicI64::new(0),
        }
    }

    pub fn new(epoch: i64, worker_id: i64, datacenter_id: i64) -> Snowflake {
        Snowflake {
            epoch,
            worker_id,
            datacenter_id,
            sequence: AtomicI64::new(0),
            time: AtomicI64::new(0),
        }
    }

    pub fn set_epoch(&mut self, epoch: i64) -> &mut Self {
        self.epoch = epoch;
        self
    }

    pub fn set_worker_id(&mut self, worker_id: i64) -> &mut Self {
        self.worker_id = worker_id;
        self
    }

    pub fn set_datacenter_id(&mut self, datacenter_id: i64) -> &mut Self {
        self.datacenter_id = datacenter_id;
        self
    }

    pub fn generate(&self) -> i64 {
        let last_timestamp = self.time.fetch_or(0, Ordering::Relaxed);
        let mut timestamp = self.get_time();
        let sequence = self.sequence.fetch_or(0, Ordering::Relaxed);
        if timestamp == last_timestamp {
            let sequence = (sequence + 1) & (-1 ^ (-1 << 12));
            self.sequence.swap(sequence, Ordering::Relaxed);
            if sequence == 0 && timestamp <= last_timestamp {
                timestamp = self.get_time();
            }
        } else {
            self.sequence.swap(0, Ordering::Relaxed);
        }
        self.time.swap(timestamp, Ordering::Relaxed);
        (timestamp << 22)
            | (self.worker_id << 17)
            | (self.datacenter_id << 12)
            | self.sequence.fetch_or(0, Ordering::Relaxed)
    }

    fn get_time(&self) -> i64 {
        Utc::now().timestamp_millis() - self.epoch
    }
}

pub static SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| Snowflake::default());

///gen new snowflake_id
pub fn new_snowflake_id() -> i64 {
    SNOWFLAKE.generate()
}

#[cfg(test)]
mod test {
    use crate::snowflake::{new_snowflake_id, Snowflake, SNOWFLAKE};

    #[test]
    fn test_gen() {
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
}
