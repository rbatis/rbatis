use crate::db::Connection;
use crate::pool::{DecrementSizeGuard, SharedPool};
use crate::Error;
use futures_core::future::BoxFuture;
use futures_intrusive::sync::SemaphoreReleaser;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Instant;

/// A connection managed by a [`Pool`][crate::pool::Pool].
///
/// Will be returned to the pool on-drop.
pub struct PoolConnection {
    pub(crate) live: Option<Live>,
    pub pool: Arc<SharedPool>,
}

pub struct Live {
    pub raw: Box<dyn Connection>,
    pub created: Instant,
}

impl Live {
    pub fn float(self, pool: Arc<SharedPool>) -> Floating<Self> {
        Floating {
            inner: self,
            // create a new guard from a previously leaked permit
            guard: Some(DecrementSizeGuard::new_permit(pool)),
        }
    }

    pub fn into_idle(self) -> Idle {
        Idle {
            live: self,
            since: Instant::now(),
        }
    }
}

pub struct Idle {
    pub live: Live,
    pub since: Instant,
}

impl Deref for Idle {
    type Target = Live;

    fn deref(&self) -> &Self::Target {
        &self.live
    }
}

impl DerefMut for Idle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.live
    }
}

/// RAII wrapper for connections being handled by functions that may drop them
pub struct Floating<C> {
    pub inner: C,
    pub guard: Option<DecrementSizeGuard>,
}

impl Floating<Live> {
    pub fn new_live(conn: Box<dyn Connection>, guard: DecrementSizeGuard) -> Self {
        Self {
            inner: Live {
                raw: conn,
                created: Instant::now(),
            },
            guard: Some(guard),
        }
    }

    pub fn attach(self, pool: &Arc<SharedPool>) -> PoolConnection {
        let Floating { inner, mut guard } = self;

        let mut guard = guard.take().expect("guard is none");
        debug_assert!(
            guard.same_pool(pool),
            "BUG: attaching connection to different pool"
        );
        guard.cancel();
        PoolConnection {
            live: Some(inner),
            pool: Arc::clone(pool),
        }
    }

    pub fn release(self) {
        if let Some(g) = &self.guard {
            g.pool.clone().release(self);
        }
    }

    pub fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
        // `guard` is dropped as intended
        let raw = self.inner.raw.close();
        Box::pin(async move { raw.await })
    }

    pub fn detach(self) -> Box<dyn Connection> {
        self.inner.raw
    }

    pub fn into_idle(self) -> Floating<Idle> {
        Floating {
            inner: self.inner.into_idle(),
            guard: self.guard,
        }
    }
}

impl Floating<Idle> {
    pub fn from_idle(idle: Idle, pool: Arc<SharedPool>, permit: SemaphoreReleaser<'_>) -> Self {
        Self {
            inner: idle,
            guard: Some(DecrementSizeGuard::from_permit(pool, permit)),
        }
    }

    pub async fn ping(&mut self) -> Result<(), Error> {
        self.live.raw.ping().await
    }

    pub fn into_live(self) -> Floating<Live> {
        Floating {
            inner: self.inner.live,
            guard: self.guard,
        }
    }

    pub fn close<'a>(&mut self) -> BoxFuture<'static, DecrementSizeGuard> {
        let c = self.inner.live.raw.close();
        let g = self.guard.take().expect("when close() on guard is none");
        Box::pin(async move {
            // `guard` is dropped as intended
            if let Err(e) = c.await {
                log::debug!("error occurred while closing the pool connection: {}", e);
            }
            g
        })
    }
}

impl<C> Deref for Floating<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C> DerefMut for Floating<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
