use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use async_trait::async_trait;
use flume::{Receiver, Sender};

pub struct ChannelPool<Manager: RBPoolManager> {
    manager: Manager,
    sender: Sender<Manager::Connection>,
    receiver: Receiver<Manager::Connection>,
    max_open: Arc<AtomicU64>,
    in_use: Arc<AtomicU64>,
}

#[async_trait]
pub trait RBPoolManager {
    type Connection: Send + 'static;

    type Error: for<'a> From<&'a str> + ToString + Send + Sync + 'static;

    async fn connect(&self) -> Result<Self::Connection, Self::Error>;
    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error>;
}

impl<M: RBPoolManager> ChannelPool<M> {
    pub fn new(m: M) -> Self {
        let (s, r) = flume::unbounded();
        Self {
            manager: m,
            sender: s,
            receiver: r,
            max_open: Arc::new(AtomicU64::new(10)),
            in_use: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn get(&self) -> Result<ConnectionBox<M>, M::Error> {
        self.get_timeout(Duration::from_secs(0)).await
    }

    pub async fn get_timeout(&self, d: Duration) -> Result<ConnectionBox<M>, M::Error> {
        let f = async {
            if self.in_use.load(Ordering::SeqCst) <= self.max_open.load(Ordering::SeqCst) {
                let conn = self.manager.connect().await?;
                self.sender.send(conn).map_err(|e| M::Error::from(&e.to_string()))?;
            }
            self.receiver.recv_async().await.map_err(|e| M::Error::from(&e.to_string()))
        };
        //TODO check connection
        self.in_use.fetch_add(1, Ordering::SeqCst);
        if d.is_zero() {
            Ok(ConnectionBox {
                inner: Some(f.await?),
                sender: self.sender.clone(),
                in_use: self.in_use.clone(),
            })
        } else {
            let out = tokio::time::timeout(d, f).await.map_err(|_e| M::Error::from("get timeout"))?;
            Ok(ConnectionBox {
                inner: Some(out?),
                sender: self.sender.clone(),
                in_use: self.in_use.clone(),
            })
        }
    }

    pub async fn state(&self) -> State {
        State{
            max_open: self.max_open.load(Ordering::Relaxed),
            connections: self.sender.len() as u64,
            in_use: self.in_use.load(Ordering::Relaxed),
        }
    }

    pub async fn set_max_open(&self, n: u64) {
        let open = self.sender.len() as u64;
        if open > n {
            let del = open - n;
            for _ in 0..del {
                _ = self.receiver.try_recv();
            }
        }
        self.max_open.store(n, Ordering::SeqCst);
    }
}

pub struct ConnectionBox<M: RBPoolManager> {
    inner: Option<M::Connection>,
    sender: Sender<M::Connection>,
    in_use: Arc<AtomicU64>,
}

impl<M: RBPoolManager> Deref for ConnectionBox<M> {
    type Target = M::Connection;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<M: RBPoolManager> DerefMut for ConnectionBox<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<M: RBPoolManager> Drop for ConnectionBox<M> {
    fn drop(&mut self) {
        if let Some(v) = self.inner.take() {
            _ = self.sender.send(v);
        }
        self.in_use.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct State {
    pub max_open: u64,
    pub connections: u64,
    pub in_use: u64,
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use crate::{ChannelPool, RBPoolManager};

    pub struct TestManager {}

    #[async_trait]
    impl RBPoolManager for TestManager {
        type Connection = i32;
        type Error = String;

        async fn connect(&self) -> Result<Self::Connection, Self::Error> {
            Ok(0)
        }

        async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
            Ok(conn)
        }
    }

    // --nocapture
    #[tokio::test]
    async fn test_pool_get() {
        let p = ChannelPool::new(TestManager {});
        let mut arr = vec![];
        for i in 0..10 {
            let v = p.get().await.unwrap();
            println!("{},{}", i, v.inner.unwrap());
            arr.push(v);
        }
    }
}
