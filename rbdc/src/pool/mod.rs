pub mod options;
pub use options::*;
pub mod conn;
pub use conn::*;

use crate::db::{ConnectOptions, Driver};
use crate::Error;
use crossbeam_queue::ArrayQueue;
use event_listener::EventListener;
use futures_core::FusedFuture;
use futures_intrusive::sync::{Semaphore, SemaphoreReleaser};
use futures_util::FutureExt;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::{cmp, mem, ptr};



///TODO maybe should use https://github.com/importcjj/mobc impl this?
///
/// Ihe number of permits to release to wake all waiters, such as on `SharedPool::close()`.
///
/// This should be large enough to realistically wake all tasks waiting on the pool without
/// potentially overflowing the permits count in the semaphore itself.
const WAKE_ALL_PERMITS: usize = usize::MAX / 2;

#[derive(Clone, Debug)]
pub struct Pool {
    inner: Arc<SharedPool>,
}

impl Pool {
    /// Creates a new connection pool with a default pool configuration and
    /// the given connection URI; and, immediately establishes one connection.
    pub async fn connect(d: Box<dyn Driver>, uri: &str) -> Result<Self, Error> {
        PoolOptions::new().connect(d, uri).await
    }
    /// Creates a new connection pool with a default pool configuration and
    /// the given connection options; and, immediately establishes one connection.
    pub async fn connect_with(options: Box<dyn ConnectOptions>) -> Result<Self, Error> {
        PoolOptions::new().connect_with(options).await
    }
    /// Creates a new connection pool with a default pool configuration and
    /// the given connection URI; and, will establish a connections as the pool
    /// starts to be used.
    pub fn connect_lazy(d: Box<dyn Driver>, uri: &str) -> Result<Self, Error> {
        PoolOptions::new().connect_lazy(d, uri)
    }

    /// Creates a new connection pool with a default pool configuration and
    /// the given connection options; and, will establish a connections as the pool
    /// starts to be used.
    pub fn connect_lazy_with(options: Box<dyn ConnectOptions>) -> Self {
        PoolOptions::new().connect_lazy_with(options)
    }

    /// Retrieves a connection from the pool.
    ///
    /// Waits for at most the configured connection timeout before returning an error.
    pub fn acquire(&self) -> impl Future<Output = Result<PoolConnection, Error>> + 'static {
        let shared = self.inner.clone();
        async move { shared.acquire().await.map(|conn| conn.attach(&shared)) }
    }

    /// Attempts to retrieve a connection from the pool if there is one available.
    ///
    /// Returns `None` immediately if there are no idle connections available in the pool.
    pub fn try_acquire(&self) -> Option<PoolConnection> {
        self.inner
            .try_acquire()
            .map(|conn| conn.into_live().attach(&self.inner))
    }
    /// Shut down the connection pool, waiting for all connections to be gracefully closed.
    ///
    /// Upon `.await`ing this call, any currently waiting or subsequent calls to [Pool::acquire] and
    /// the like will immediately return [Error::PoolClosed] and no new connections will be opened.
    ///
    /// Any connections currently idle in the pool will be immediately closed, including sending
    /// a graceful shutdown message to the database server, if applicable.
    ///
    /// Checked-out connections are unaffected, but will be closed in the same manner when they are
    /// returned to the pool.
    ///
    /// Does not resolve until all connections are returned to the pool and gracefully closed.
    ///
    /// ### Note: `async fn`
    /// Because this is an `async fn`, the pool will *not* be marked as closed unless the
    /// returned future is polled at least once.
    ///
    /// If you want to close the pool but don't want to wait for all connections to be gracefully
    /// closed, you can do `pool.close().now_or_never()`, which polls the future exactly once
    /// with a no-op waker.
    // TODO: I don't want to change the signature right now in case it turns out to be a
    // breaking change, but this probably should eagerly mark the pool as closed and then the
    // returned future only needs to be awaited to gracefully close the connections.
    pub async fn close(&self) {
        self.inner.close().await;
    }

    /// Returns `true` if [`.close()`][Pool::close] has been called on the pool, `false` otherwise.
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Get a future that resolves when [`Pool::close()`] is called.
    ///
    /// If the pool is already closed, the future resolves immediately.
    ///
    /// This can be used to cancel long-running operations that hold onto a [`PoolConnection`]
    /// so they don't prevent the pool from closing (which would otherwise wait until all
    /// connections are returned).
    ///
    /// Examples
    /// ========
    /// These examples use Postgres and Tokio, but should suffice to demonstrate the concept.
    ///
    /// Do something when the pool is closed:
    /// ```rust,no_run
    /// # #[cfg(feature = "postgres")]
    /// # async fn bleh() -> sqlx_core::error::Result<()> {
    /// use sqlx::PgPool;
    ///
    /// let pool = PgPool::connect("postgresql://...").await?;
    ///
    /// let pool2 = pool.clone();
    ///
    /// tokio::spawn(async move {
    ///     // Demonstrates that `CloseEvent` is itself a `Future` you can wait on.
    ///     // This lets you implement any kind of on-close event that you like.
    ///     pool2.close_event().await;
    ///
    ///     println!("Pool is closing!");
    ///
    ///     // Imagine maybe recording application statistics or logging a report, etc.
    /// });
    ///
    /// // The rest of the application executes normally...
    ///
    /// // Close the pool before the application exits...
    /// pool.close().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Cancel a long-running operation:
    /// ```rust,no_run
    /// # #[cfg(feature = "postgres")]
    /// # async fn bleh() -> sqlx_core::error::Result<()> {
    /// use sqlx::{Executor, PgPool};
    ///
    /// let pool = PgPool::connect("postgresql://...").await?;
    ///
    /// let pool2 = pool.clone();
    ///
    /// tokio::spawn(async move {
    ///     pool2.close_event().do_until(async {
    ///         // This statement normally won't return for 30 days!
    ///         // (Assuming the connection doesn't time out first, of course.)
    ///         pool2.execute("SELECT pg_sleep('30 days')").await;
    ///
    ///         // If the pool is closed before the statement completes, this won't be printed.
    ///         // This is because `.do_until()` cancels the future it's given if the
    ///         // pool is closed first.
    ///         println!("Waited!");
    ///     }).await;
    /// });
    ///
    /// // This normally wouldn't return until the above statement completed and the connection
    /// // was returned to the pool. However, thanks to `.do_until()`, the operation was
    /// // cancelled as soon as we called `.close().await`.
    /// pool.close().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn close_event(&self) -> CloseEvent {
        CloseEvent {
            listener: (!self.is_closed()).then(|| self.inner.on_closed.listen()),
        }
    }

    /// Returns the number of connections currently active. This includes idle connections.
    pub fn size(&self) -> u32 {
        self.inner.size()
    }

    /// Returns the number of connections active and idle (not in use).
    ///
    /// This will block until the number of connections stops changing for at
    /// least 2 atomic accesses in a row. If the number of idle connections is
    /// changing rapidly, this may run indefinitely.
    pub fn num_idle(&self) -> usize {
        self.inner.num_idle()
    }
}

impl From<SharedPool> for Pool {
    fn from(p: SharedPool) -> Self {
        Self { inner: Arc::new(p) }
    }
}
impl From<Arc<SharedPool>> for Pool {
    fn from(p: Arc<SharedPool>) -> Self {
        Self { inner: p }
    }
}

/// A future that resolves when the pool is closed.
///
/// See [`Pool::close_event()`] for details.
pub struct CloseEvent {
    listener: Option<EventListener>,
}

impl CloseEvent {
    /// Execute the given future until it returns or the pool is closed.
    ///
    /// Cancels the future and returns `Err(PoolClosed)` if/when the pool is closed.
    /// If the pool was already closed, the future is never run.
    pub async fn do_until<Fut: Future>(&mut self, fut: Fut) -> Result<Fut::Output, Error> {
        // Check that the pool wasn't closed already.
        //
        // We use `poll_immediate()` as it will use the correct waker instead of
        // a no-op one like `.now_or_never()`, but it won't actually suspend execution here.
        futures_util::future::poll_immediate(&mut *self)
            .await
            .map_or(Ok(()), |_| Err(Error::from("PoolClosed")))?;

        futures_util::pin_mut!(fut);

        // I find that this is clearer in intent than `futures_util::future::select()`
        // or `futures_util::select_biased!{}` (which isn't enabled anyway).
        futures_util::future::poll_fn(|cx| {
            // Poll `fut` first as the wakeup event is more likely for it than `self`.
            if let Poll::Ready(ret) = fut.as_mut().poll(cx) {
                return Poll::Ready(Ok(ret));
            }

            // Can't really factor out mapping to `Err(Error::PoolClosed)` though it seems like
            // we should because that results in a different `Ok` type each time.
            //
            // Ideally we'd map to something like `Result<!, Error>` but using `!` as a type
            // is not allowed on stable Rust yet.
            self.poll_unpin(cx).map(|_| Err(Error::from("PoolClosed")))
        })
        .await
    }
}

impl Future for CloseEvent {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(listener) = &mut self.listener {
            futures_core::ready!(listener.poll_unpin(cx));
        }

        // `EventListener` doesn't like being polled after it yields, and even if it did it
        // would probably just wait for the next event, neither of which we want.
        //
        // So this way, once we get our close event, we fuse this future to immediately return.
        self.listener = None;

        Poll::Ready(())
    }
}

impl FusedFuture for CloseEvent {
    fn is_terminated(&self) -> bool {
        self.listener.is_none()
    }
}

pub struct SharedPool {
    pub connect_options: Box<dyn ConnectOptions>,
    pub idle_conns: ArrayQueue<Idle>,
    pub semaphore: Semaphore,
    pub size: AtomicU32,
    is_closed: AtomicBool,
    pub on_closed: event_listener::Event,
    pub options: PoolOptions,
}

impl SharedPool {
    pub fn new_arc(options: PoolOptions, connect_options: Box<dyn ConnectOptions>) -> Arc<Self> {
        let capacity = options.max_connections as usize;

        // ensure the permit count won't overflow if we release `WAKE_ALL_PERMITS`
        // this assert should never fire on 64-bit targets as `max_connections` is a u32
        let _ = capacity
            .checked_add(WAKE_ALL_PERMITS)
            .expect("max_connections exceeds max capacity of the pool");

        let pool = Self {
            connect_options,
            idle_conns: ArrayQueue::new(capacity),
            semaphore: Semaphore::new(options.fair, capacity),
            size: AtomicU32::new(0),
            is_closed: AtomicBool::new(false),
            on_closed: event_listener::Event::new(),
            options,
        };

        let pool = Arc::new(pool);

        spawn_reaper(&pool);

        pool
    }
    pub fn size(&self) -> u32 {
        self.size.load(Ordering::Acquire)
    }

    pub fn num_idle(&self) -> usize {
        // NOTE: This is very expensive
        self.idle_conns.len()
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(Ordering::Acquire)
    }

    pub async fn close(self: &Arc<Self>) {
        let already_closed = self.is_closed.swap(true, Ordering::AcqRel);

        if !already_closed {
            // if we were the one to mark this closed, release enough permits to wake all waiters
            // we can't just do `usize::MAX` because that would overflow
            // and we can't do this more than once cause that would _also_ overflow
            self.semaphore.release(WAKE_ALL_PERMITS);
            self.on_closed.notify(usize::MAX);
        }

        // wait for all permits to be released
        let _permits = self
            .semaphore
            .acquire(WAKE_ALL_PERMITS + (self.options.max_connections as usize))
            .await;

        while let Some(idle) = self.idle_conns.pop() {
            let _ = idle.live.float((*self).clone()).close().await;
        }
    }

    #[inline]
    pub fn try_acquire(self: &Arc<Self>) -> Option<Floating<Idle>> {
        if self.is_closed() {
            return None;
        }

        let permit = self.semaphore.try_acquire(1)?;
        self.pop_idle(permit).ok()
    }

    fn pop_idle<'a>(
        self: &'a Arc<Self>,
        permit: SemaphoreReleaser<'a>,
    ) -> Result<Floating<Idle>, SemaphoreReleaser<'a>> {
        if let Some(idle) = self.idle_conns.pop() {
            Ok(Floating::from_idle(idle, (*self).clone(), permit))
        } else {
            Err(permit)
        }
    }

    pub fn release(&self, mut floating: Floating<Live>) {
        if let Some(test) = &self.options.after_release {
            if !test(&mut floating.raw) {
                // drop the connection and do not return it to the pool
                return;
            }
        }

        let Floating { inner: idle, guard } = floating.into_idle();

        if !self.idle_conns.push(idle).is_ok() {
            panic!("BUG: connection queue overflow in release()");
        }

        // NOTE: we need to make sure we drop the permit *after* we push to the idle queue
        // don't decrease the size
        guard.expect("release() guard is none!").release_permit();
    }

    /// Try to atomically increment the pool size for a new connection.
    ///
    /// Returns `None` if we are at max_connections or if the pool is closed.
    pub fn try_increment_size<'a>(
        self: &'a Arc<Self>,
        permit: SemaphoreReleaser<'a>,
    ) -> Result<DecrementSizeGuard, SemaphoreReleaser<'a>> {
        match self
            .size
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |size| {
                size.checked_add(1)
                    .filter(|size| size <= &self.options.max_connections)
            }) {
            // we successfully incremented the size
            Ok(_) => Ok(DecrementSizeGuard::from_permit((*self).clone(), permit)),
            // the pool is at max capacity
            Err(_) => Err(permit),
        }
    }

    #[allow(clippy::needless_lifetimes)]
    pub async fn acquire(self: &Arc<Self>) -> Result<Floating<Live>, Error> {
        if self.is_closed() {
            return Err(Error::from("PoolClosed"));
        }

        let deadline = Instant::now() + self.options.connect_timeout;

        crate::rt::timeout(
            self.options.connect_timeout,
            async {
                loop {
                    let permit = self.semaphore.acquire(1).await;

                    if self.is_closed() {
                        return Err(Error::from("PoolClosed"));
                    }

                    // First attempt to pop a connection from the idle queue.
                    let guard = match self.pop_idle(permit) {

                        // Then, check that we can use it...
                        Ok(conn) => match check_conn(conn, &self.options).await {

                            // All good!
                            Ok(live) => return Ok(live),

                            // if the connection isn't usable for one reason or another,
                            // we get the `DecrementSizeGuard` back to open a new one
                            Err(guard) => guard,
                        },
                        Err(permit) => if let Ok(guard) = self.try_increment_size(permit) {
                            // we can open a new connection
                            guard
                        } else {
                            log::debug!("woke but was unable to acquire idle connection or open new one; retrying");
                            continue;
                        }
                    };

                    // Attempt to connect...
                    return self.connection(deadline, guard).await;
                }
            }
        )
            .await
            .map_err(|_| Error::from("PoolTimedOut"))?
    }

    pub async fn connection(
        self: &Arc<Self>,
        deadline: Instant,
        guard: DecrementSizeGuard,
    ) -> Result<Floating<Live>, Error> {
        if self.is_closed() {
            return Err(Error::from("PoolClosed"));
        }

        let mut backoff = Duration::from_millis(10);
        let max_backoff = deadline_as_timeout(deadline)? / 5;

        loop {
            let timeout = deadline_as_timeout(deadline)?;

            // result here is `Result<Result<C, Error>, TimeoutError>`
            // if this block does not return, sleep for the backoff timeout and try again
            match crate::rt::timeout(timeout, self.connect_options.connect()).await {
                // successfully established connection
                Ok(Ok(mut raw)) => {
                    if let Some(callback) = &self.options.after_connect {
                        callback(&mut raw).await?;
                    }

                    return Ok(Floating::new_live(raw, guard));
                }

                // an IO error while connecting is assumed to be the system starting up
                Ok(Err(Error::Io(e))) if e.kind() == std::io::ErrorKind::ConnectionRefused => (),

                // TODO: Handle other database "boot period"s

                // [postgres] the database system is starting up
                // TODO: Make this check actually check if this is postgres
                Ok(Err(Error::E(error))) if error.contains("57P03") => (),

                // Any other error while connection should immediately
                // terminate and bubble the error up
                Ok(Err(e)) => return Err(e),

                // timed out
                Err(_) => return Err(Error::from("PoolTimedOut")),
            }

            // If the connection is refused wait in exponentially
            // increasing steps for the server to come up,
            // capped by a factor of the remaining time until the deadline
            crate::rt::sleep(backoff).await;
            backoff = cmp::min(backoff * 2, max_backoff);
        }
    }
}

// NOTE: Function names here are bizarre. Helpful help would be appreciated.

fn is_beyond_lifetime(live: &Live, options: &PoolOptions) -> bool {
    // check if connection was within max lifetime (or not set)
    options
        .max_lifetime
        .map_or(false, |max| live.created.elapsed() > max)
}

fn is_beyond_idle(idle: &Idle, options: &PoolOptions) -> bool {
    // if connection wasn't idle too long (or not set)
    options
        .idle_timeout
        .map_or(false, |timeout| idle.since.elapsed() > timeout)
}

async fn check_conn(
    mut conn: Floating<Idle>,
    options: &PoolOptions,
) -> Result<Floating<Live>, DecrementSizeGuard> {
    // If the connection we pulled has expired, close the connection and
    // immediately create a new connection
    if is_beyond_lifetime(&conn, options) {
        // we're closing the connection either way
        // close the connection but don't really care about the result
        return Err(conn.close().await);
    } else if options.test_before_acquire {
        // Check that the connection is still live
        if let Err(e) = conn.ping().await {
            // an error here means the other end has hung up or we lost connectivity
            // either way we're fine to just discard the connection
            // the error itself here isn't necessarily unexpected so WARN is too strong
            log::info!("ping on idle connection returned error: {}", e);
            // connection is broken so don't try to close nicely
            return Err(conn.close().await);
        }
    } else if let Some(test) = &options.before_acquire {
        match test(&mut conn.live.raw).await {
            Ok(false) => {
                // connection was rejected by user-defined hook
                return Err(conn.close().await);
            }

            Err(error) => {
                log::info!("in `before_acquire`: {}", error);
                return Err(conn.close().await);
            }

            Ok(true) => {}
        }
    }

    // No need to re-connect; connection is alive or we don't care
    Ok(conn.into_live())
}

/// if `max_lifetime` or `idle_timeout` is set, spawn a task that reaps senescent connections
fn spawn_reaper(pool: &Arc<SharedPool>) {
    let period = match (pool.options.max_lifetime, pool.options.idle_timeout) {
        (Some(it), None) | (None, Some(it)) => it,

        (Some(a), Some(b)) => cmp::min(a, b),

        (None, None) => return,
    };

    let pool = Arc::clone(&pool);

    crate::rt::spawn(async move {
        while !pool.is_closed() {
            if !pool.idle_conns.is_empty() {
                do_reap(&pool).await;
            }
            crate::rt::sleep(period).await;
        }
    });
}

async fn do_reap(pool: &Arc<SharedPool>) {
    // reap at most the current size minus the minimum idle
    let max_reaped = pool.size().saturating_sub(pool.options.min_connections);

    // collect connections to reap
    let (reap, keep) = (0..max_reaped)
        // only connections waiting in the queue
        .filter_map(|_| pool.try_acquire())
        .partition::<Vec<_>, _>(|conn| {
            is_beyond_idle(conn, &pool.options) || is_beyond_lifetime(conn, &pool.options)
        });

    for conn in keep {
        // return valid connections to the pool first
        pool.release(conn.into_live());
    }

    for mut conn in reap {
        let _ = conn.close().await;
    }
}

impl Debug for SharedPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedPool")
            .field("connect_options", &self.connect_options)
            .field("idle_conns", &self.idle_conns.len())
            .field("size", &self.size())
            .field("is_closed", &self.is_closed())
            .field("options", &self.options)
            .finish()
    }
}

/// RAII guard returned by `Pool::try_increment_size()` and others.
///
/// Will decrement the pool size if dropped, to avoid semantically "leaking" connections
/// (where the pool thinks it has more connections than it does).
pub struct DecrementSizeGuard {
    pub pool: Arc<SharedPool>,
    dropped: bool,
}

impl DecrementSizeGuard {
    /// Create a new guard that will release a semaphore permit on-drop.
    pub fn new_permit(pool: Arc<SharedPool>) -> Self {
        Self {
            pool,
            dropped: false,
        }
    }

    pub fn from_permit(pool: Arc<SharedPool>, mut permit: SemaphoreReleaser<'_>) -> Self {
        // here we effectively take ownership of the permit
        permit.disarm();
        Self::new_permit(pool)
    }

    /// Return `true` if the internal references point to the same fields in `SharedPool`.
    pub fn same_pool(&self, pool: &SharedPool) -> bool {
        ptr::eq(&*self.pool, pool)
    }

    /// Release the semaphore permit without decreasing the pool size.
    fn release_permit(self) {
        self.pool.semaphore.release(1);
        self.cancel();
    }

    pub fn cancel(self) {
        mem::forget(self);
    }
}

/// get the time between the deadline and now and use that as our timeout
///
/// returns `Error::PoolTimedOut` if the deadline is in the past
fn deadline_as_timeout(deadline: Instant) -> Result<Duration, Error> {
    deadline
        .checked_duration_since(Instant::now())
        .ok_or(Error::from("PoolTimedOut"))
}

impl Drop for DecrementSizeGuard {
    fn drop(&mut self) {
        assert!(!self.dropped, "double-dropped!");
        self.dropped = true;
        self.pool.size.fetch_sub(1, Ordering::SeqCst);

        // and here we release the permit we got on construction
        self.pool.semaphore.release(1);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_pool() {}
}
