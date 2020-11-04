use std::time::{SystemTime, Instant};
use rustflake::Snowflake;
use std::sync::{Mutex, MutexGuard, TryLockError};
use rbatis_core::runtime::Mutex as AsyncMutex;

lazy_static!(
   pub static ref SNOWFLAKE:Mutex<Snowflake> = Mutex::new(Snowflake::default());
   pub static ref ASYNC_SNOWFLAKE:AsyncMutex<Snowflake> = AsyncMutex::new(Snowflake::default());
);

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

#[cfg(test)]
mod test {
    use crate::plugin::snowflake::{block_snowflake_id, async_snowflake_id};

    #[test]
    fn test_new_block_id() {
        println!("{}", block_snowflake_id());
        println!("{}", block_snowflake_id());
    }

    #[test]
    fn test_new_async_id() {
        async_std::task::block_on(async {
            println!("{}", async_snowflake_id().await);
            println!("{}", async_snowflake_id().await);
        });
    }
}