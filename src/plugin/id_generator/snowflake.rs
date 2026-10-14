//! Snowflake id generator (i64).
//!
//! A snowflake id packs a timestamp, a machine id, a node id and a per-millisecond
//! counter into one `i64`, which makes ids unique across machines without a
//! database round trip:
//!
//! ```rust
//! use rbatis::id_generator::{IdGenerator, Snowflake};
//!
//! let snowflake = Snowflake::new(1, 1, 0); // machine_id, node_id, mode
//! let id: i64 = snowflake.generate();
//! assert!(id > 0);
//! ```
//!
//! `mode` is `0` for the fast path (the timestamp is only advanced when the id
//! space of a millisecond is exhausted) or `1` for real time generation.

use parking_lot::ReentrantMutex;
use std::hint::spin_loop;
use std::sync::atomic::{AtomicI64, AtomicU16, Ordering};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// ID generator trait for generating unique task IDs
pub trait IdGenerator: Send + Sync + std::fmt::Debug {
    /// Generate a unique ID (i64)
    fn generate(&self) -> i64;
}

/// Snowflake id generator state.
///
/// The fields are public so a generator can be tuned or cloned; the whole struct is
/// `Send + Sync` and `generate_id()` is safe to call from several tasks.
#[derive(Debug)]
pub struct Snowflake {
    /// start of the timestamp, defaults to [`UNIX_EPOCH`] (41 bits of the id)
    pub epoch: SystemTime,
    /// last used timestamp in milliseconds
    pub last_timestamp: AtomicI64, // Atomic to replace mutable i64
    /// machine id (5 bits of the id)
    pub machine_id: i32,
    /// node id (5 bits of the id)
    pub node_id: i32,
    /// `0` = fast generate, `1` = realtime generate
    pub mode: i32,      // mode [0=fast_generate,1=realtime_generate]
    /// per-millisecond counter (12 bits of the id)
    pub idx: AtomicU16, // Atomic to replace mutable u16
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
    /// create a generator using [`UNIX_EPOCH`] as epoch, see [`Snowflake::with_epoch`]
    pub fn new(machine_id: i32, node_id: i32, mode: i32) -> Snowflake {
        Self::with_epoch(machine_id, node_id, mode, UNIX_EPOCH)
    }

    /// from epoch for example:
    ///
    /// ```rust
    /// use std::time::UNIX_EPOCH;
    /// use rbatis::id_generator::Snowflake;
    /// let snowflake = Snowflake::with_epoch(1,1,0,UNIX_EPOCH);
    /// ```
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

    /// from last_timestamp for example:
    ///
    /// ```rust
    /// use std::time::UNIX_EPOCH;
    /// use rbatis::id_generator::Snowflake;
    /// let snowflake = Snowflake::with_last_timestamp(1,1,0, 1726417159);
    /// ```
    pub fn with_last_timestamp(
        machine_id: i32,
        node_id: i32,
        mode: i32,
        last_timestamp: i64,
    ) -> Snowflake {
        Snowflake {
            epoch: UNIX_EPOCH,
            last_timestamp: AtomicI64::new(last_timestamp),
            machine_id,
            node_id,
            mode,
            idx: AtomicU16::new(0),
            lock: ReentrantMutex::new(()),
        }
    }

    /// a generator with `machine_id = 1`, `node_id = 1`, `mode = 0` and `UNIX_EPOCH`
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Snowflake {
        <Snowflake as Default>::default()
    }
}

impl Default for Snowflake {
    fn default() -> Self {
        Snowflake::new(1, 1, 0)
    }
}

impl Snowflake {
    /// generate the next id (`i64`)
    #[inline]
    pub fn generate_id(&self) -> i64 {
        let g = self.lock.lock();
        let idx = self.idx.fetch_add(1, Ordering::SeqCst) % 4096;
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
        }
        let last_time = self.last_timestamp.load(Ordering::SeqCst);
        drop(g);
        last_time << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (idx as i64)
    }

    /// milliseconds elapsed since `epoch`
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

/// Note:  if you have multiple machines/server, please modify the machine ID and node ID
pub static SNOWFLAKE: LazyLock<Snowflake> = LazyLock::new(|| Snowflake::new(1, 1, 0));

///gen new snowflake_id
pub fn new_snowflake_id() -> i64 {
    SNOWFLAKE.generate_id()
}

impl IdGenerator for Snowflake {
    fn generate(&self) -> i64 {
        self.generate_id()
    }
}
