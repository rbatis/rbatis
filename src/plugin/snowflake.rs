use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::{Instant, SystemTime};

use rustflake::Snowflake;

use crate::core::runtime::Mutex as AsyncMutex;

lazy_static! {
    pub static ref SNOWFLAKE: Mutex<Snowflake> = Mutex::new(Snowflake::default());
    pub static ref ASYNC_SNOWFLAKE: AsyncMutex<Snowflake> = AsyncMutex::new(Snowflake::default());
};

/// return an snowflake id,this method use async Mutex
pub async fn async_snowflake_id() -> i64 {
    ASYNC_SNOWFLAKE.lock().await.generate()
}

///if lock fail,will return -1
pub fn block_snowflake_id() -> i64 {
    match SNOWFLAKE.lock() {
        Ok(mut v) => {
            return v.generate();
        }
        _ => {
            return -1;
        }
    }
}
