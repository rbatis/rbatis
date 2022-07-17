use crate::db::{ConnectOptions, Connection};
use crate::Error;
use crossbeam_queue::ArrayQueue;
use futures_core::future::BoxFuture;
use futures_intrusive::sync::{Semaphore, SemaphoreReleaser};
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub(crate) struct SharedPool {
    pub(super) connect_options: Box<dyn ConnectOptions>,
    pub(super) idle_conns: ArrayQueue<Idle>,
    pub(super) semaphore: Semaphore,
    pub(super) size: AtomicU32,
    is_closed: AtomicBool,
    pub(super) on_closed: event_listener::Event,
    pub(super) options: PoolOptions,
}

pub struct PoolOptions {
    pub(crate) test_before_acquire: bool,
    pub(crate) after_connect: Option<
        Box<
            dyn Fn(&mut dyn Connection) -> BoxFuture<'_, Result<(), Error>> + 'static + Send + Sync,
        >,
    >,
    pub(crate) before_acquire: Option<
        Box<
            dyn Fn(&mut dyn Connection) -> BoxFuture<'_, Result<bool, Error>>
                + 'static
                + Send
                + Sync,
        >,
    >,
    pub(crate) after_release:
        Option<Box<dyn Fn(&mut dyn Connection) -> bool + 'static + Send + Sync>>,
    pub(crate) max_connections: u32,
    pub(crate) connect_timeout: Duration,
    pub(crate) min_connections: u32,
    pub(crate) max_lifetime: Option<Duration>,
    pub(crate) idle_timeout: Option<Duration>,
    pub(crate) fair: bool,
}

/// A connection managed by a [`Pool`][crate::pool::Pool].
///
/// Will be returned to the pool on-drop.
pub struct PoolConnection {
    live: Option<Live>,
    pub(crate) pool: Arc<SharedPool>,
}

pub(super) struct Live {
    pub(super) raw: Box<dyn Connection>,
    pub(super) created: Instant,
}

pub(super) struct Idle {
    pub(super) live: Live,
    pub(super) since: Instant,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_pool() {}
}
