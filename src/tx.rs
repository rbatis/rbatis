use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use crate::core::db::DBPool;
use crate::core::db::DBTx;
use crate::plugin::log::LogPlugin;
use crate::rbatis::Rbatis;
use std::future::Future;
use std::env::Args;
use std::pin::Pin;
use std::borrow::Borrow;
use crate::core::runtime::sync::{Mutex, MutexGuard, RwLock};

#[derive(Debug)]
pub struct Tx {
    pub tx_id: String,
    pub tx: DBTx,
    pub instant: Instant,
}

///the Transaction manager，It manages the life cycle of transactions and provides access across threads
///every tx_check_interval check tx is out of time(tx_lock_wait_timeout).if out, rollback tx.
///if tx manager will be drop, manager will rollback all of tx.
#[derive(Debug)]
pub struct TxManager {
    pub tx_prefix: String,
    pub tx_timeout: RwLock<HashMap<String, Instant>>,
    pub tx_context: RwLock<HashMap<String, Mutex<Tx>>>,
    pub tx_lock_wait_timeout: Duration,
    pub tx_check_interval: Duration,
    pub alive: AtomicBool,
    pub log_plugin: Option<Arc<Box<dyn LogPlugin>>>,
}

impl Drop for TxManager {
    fn drop(&mut self) {
        self.set_alive(false);
    }
}

impl TxManager {
    pub fn new_arc(
        tx_prefix: &str,
        plugin: Arc<Box<dyn LogPlugin>>,
        tx_lock_wait_timeout: Duration,
        tx_check_interval: Duration,
    ) -> Arc<Self> {
        let m = Self {
            tx_prefix: tx_prefix.to_string(),
            tx_context: RwLock::new(HashMap::new()),
            tx_timeout: RwLock::new(HashMap::new()),
            tx_lock_wait_timeout,
            tx_check_interval: Default::default(),
            alive: AtomicBool::new(true),
            log_plugin: Some(plugin),
        };
        let arc = Arc::new(m);

        let arc_clone = arc.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(arc_clone.tx_lock_wait_timeout);
                crate::core::runtime::task::block_on(async{
                    let guard= arc_clone.tx_timeout.read().await;
                    for (tx_id,value) in guard.iter() {
                        if arc_clone.get_alive().eq(&false) {
                            break;
                        }
                        if value.elapsed().gt(&arc_clone.tx_lock_wait_timeout) {
                            futures::executor::block_on(async {
                                let is_ok = arc_clone.rollback(&tx_id).await;
                                #[cfg(feature = "debug_mode")]
                                    {
                                        log::error!("[rbatis] tx:{} rollback error: {}", tx_id, is_ok.err().unwrap());
                                    }
                            })
                        }
                    }
                });
                if arc_clone.get_alive().eq(&false) {
                    drop(arc_clone);
                    break;
                }
            }
        });
        arc
    }

    pub fn set_alive(&self, alive: bool) {
        self.alive
            .compare_exchange(!alive, alive, Ordering::Relaxed, Ordering::Relaxed);
    }

    pub fn get_alive(&self) -> bool {
        self.alive.fetch_or(false, Ordering::Relaxed)
    }

    pub fn close(&self) {
        if self.get_alive().eq(&true) {
            self.set_alive(false);
        }
    }

    fn is_enable_log(&self) -> bool {
        self.log_plugin.is_some() && self.log_plugin.as_ref().unwrap().is_enable()
    }

    fn do_log(&self, context_id: &str, arg: &str) {
        if self.is_enable_log() {
            match &self.log_plugin {
                Some(v) => {
                    v.info(context_id, arg);
                }
                _ => {}
            }
        }
    }

    /// begin tx,for new conn
    pub async fn begin(
        &self,
        new_context_id: &str,
        pool: &DBPool,
    ) -> Result<String, crate::core::Error> {
        if new_context_id.is_empty() {
            return Err(crate::core::Error::from(
                "[rbatis] context_id can not be empty",
            ));
        }
        let conn: DBTx = pool.begin().await?;
        let tx = Tx {
            tx_id: new_context_id.to_string(),
            tx: conn,
            instant: Instant::now(),
        };
        self.tx_timeout.write().await.insert(new_context_id.to_string(), tx.instant.clone());
        self.tx_context.write().await
            .insert(
                new_context_id.to_string(),
                Mutex::new(tx),
            );
        if self.is_enable_log() {
            self.do_log(
                new_context_id,
                &format!("[rbatis] [{}] Begin", new_context_id),
            );
        }
        return Ok(new_context_id.to_string());
    }

    /// commit tx,and return conn
    pub async fn commit(&self, context_id: &str) -> Result<String, crate::core::Error> {
        let tx = self.tx_context.write().await.remove(context_id).ok_or_else(||{
            crate::core::Error::from(format!(
                "[rbatis] tx:{} not exist！",
                context_id
            ))
        })?;
        let result = tx.lock().await.tx.commit().await?;
        self.tx_timeout.write().await.remove(context_id);
        if self.is_enable_log() {
            self.do_log(context_id, &format!("[rbatis] [{}] Commit", context_id));
        }
        return Ok(context_id.to_string());
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, context_id: &str) -> Result<String, crate::core::Error> {
        let tx = self.tx_context.write().await.remove(context_id).ok_or_else( ||{
            crate::core::Error::from(format!(
                "[rbatis] tx:{} not exist！",
                context_id
            ))
        })?;
        let result = tx.into_inner().tx.rollback().await?;
        self.tx_timeout.write().await.remove(context_id);
        if self.is_enable_log() {
            self.do_log(context_id, &format!("[rbatis] [{}] Rollback", context_id));
        }
        return Ok(context_id.to_string());
    }

    /// context_id is 'tx:' prefix?
    pub fn is_tx_id(&self, context_id: &str) -> bool {
        return context_id.starts_with(&self.tx_prefix);
    }
}

/// the TxGuard just like an  Lock Guard,
/// if TxGuard Drop, this tx will be commit or rollback
#[derive(Debug)]
pub struct TxGuard {
    pub tx_id: String,
    pub is_drop_commit: bool,
    pub manager: Option<Arc<TxManager>>,
}

impl TxGuard {
    pub fn new(tx_id: &str, is_drop_commit: bool, manager: Arc<TxManager>) -> Self {
        Self {
            tx_id: tx_id.to_string(),
            is_drop_commit,
            manager: Some(manager),
        }
    }

    pub async fn try_commit(&mut self) -> Result<String, crate::core::Error> {
        match &mut self.manager {
            Some(m) => {
                let result = m.commit(&self.tx_id).await?;
                self.manager = None;
                return Ok(result);
            }
            _ => {}
        }
        return Result::Ok(self.tx_id.clone());
    }

    pub async fn try_rollback(&mut self) -> Result<String, crate::core::Error> {
        match &mut self.manager {
            Some(m) => {
                let result = m.rollback(&self.tx_id).await?;
                self.manager = None;
                return Ok(result);
            }
            _ => {}
        }
        return Result::Ok(self.tx_id.clone());
    }
}

impl Drop for TxGuard {
    fn drop(&mut self) {
        if self.manager.is_none() {
            return;
        }
        let tx_id = self.tx_id.clone();
        let is_drop_commit = self.is_drop_commit;
        let manager = self.manager.take().unwrap();
        crate::core::runtime::task::spawn(async move {
            if is_drop_commit {
                manager.commit(&tx_id).await;
            } else {
                manager.rollback(&tx_id).await;
            }
            drop(manager);
        });
    }
}
