use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use crate::core::db::DBPool;
use crate::core::db::DBTx;
use crate::core::runtime::channel::{Receiver, Sender};
use crate::core::runtime::sync::{RwLock, RwLockReadGuard};
use crate::core::sync::sync_map::{RefMut, SyncMap};
use crate::plugin::log::LogPlugin;
use crate::rbatis::Rbatis;
use std::sync::Arc;

///the Transaction manager，It manages the life cycle of transactions and provides access across threads
///every tx_check_interval check tx is out of time(tx_lock_wait_timeout).if out, rollback tx.
///if tx manager will be drop, manager will rollback all of tx.
#[derive(Debug)]
pub struct TxManager {
    pub tx_prefix: String,
    pub tx_context: SyncMap<String, (DBTx, TxState)>,
    pub tx_lock_wait_timeout: Duration,
    pub tx_check_interval: Duration,
    alive: RwLock<bool>,
    close_sender: Sender<bool>,
    close_recv: Receiver<bool>,
    pub log_plugin: Option<Arc<Box<dyn LogPlugin>>>,
}

#[derive(Debug)]
pub enum TxState {
    StateBegin(Instant),
    StateFinish(Instant),
}

impl TxManager {
    pub fn new_arc(
        tx_prefix: &str,
        plugin: Arc<Box<dyn LogPlugin>>,
        tx_lock_wait_timeout: Duration,
        tx_check_interval: Duration,
    ) -> Arc<Self> {
        let (s, r) = crate::core::runtime::channel::bounded(1);
        let s = Self {
            tx_prefix: tx_prefix.to_string(),
            tx_context: SyncMap::new(),
            tx_lock_wait_timeout,
            tx_check_interval,
            alive: RwLock::new(false),
            close_sender: s,
            close_recv: r,
            log_plugin: Some(plugin),
        };
        let arc = Arc::new(s);
        TxManager::polling_check(arc.clone());
        arc
    }

    async fn set_alive(&self, alive: bool) {
        let mut l = self.alive.write().await;
        *l = alive;
    }

    pub async fn get_alive(&self) -> RwLockReadGuard<'_, bool> {
        self.alive.read().await
    }

    pub async fn close(&self) {
        if self.get_alive().await.eq(&true) {
            self.set_alive(false).await;
            let r = self.close_recv.recv().await;
        }
    }

    fn is_enable_log(&self) -> bool {
        self.log_plugin.is_some() && self.log_plugin.as_ref().unwrap().is_enable()
    }

    fn do_log(&self, context_id: &str, arg: &str) {
        if self.is_enable_log() {
            match &self.log_plugin {
                Some(v) => {
                    v.do_log(context_id, arg);
                }
                _ => {}
            }
        }
    }

    ///polling check tx alive
    fn polling_check(manager: Arc<Self>) {
        crate::core::runtime::task::spawn(async move {
            loop {
                if manager.get_alive().await.deref() == &false {
                    //rollback all
                    let m = manager.tx_context.read().await;
                    let mut rollback_ids = vec![];
                    for (k, (tx, state)) in m.deref() {
                        rollback_ids.push(k.to_string());
                    }
                    drop(m);
                    for context_id in &rollback_ids {
                        if manager.is_enable_log() {
                            manager.do_log(
                                context_id,
                                &format!(
                                    "[rbatis] rollback context_id:{},Because the manager exits",
                                    context_id
                                ),
                            );
                        }
                        manager.rollback(context_id).await;
                    }
                    //notice close
                    manager.close_sender.send(true);
                    return;
                }
                let m = manager.tx_context.read().await;
                let mut need_rollback = None;
                for (k, (tx, state)) in m.deref() {
                    match state {
                        TxState::StateBegin(instant) => {
                            let out_time = instant.elapsed();
                            if out_time > manager.tx_lock_wait_timeout {
                                if need_rollback == None {
                                    need_rollback = Some(vec![]);
                                }
                                match &mut need_rollback {
                                    Some(v) => {
                                        v.push(k.to_string());
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                drop(m);
                match &mut need_rollback {
                    Some(v) => {
                        for context_id in v {
                            if manager.is_enable_log() {
                                manager.do_log(
                                    context_id,
                                    &format!(
                                        "[rbatis] rollback context_id:{},out of time:{:?}",
                                        context_id, &manager.tx_lock_wait_timeout
                                    ),
                                );
                            }
                            manager.rollback(context_id).await;
                        }
                        //shrink_to_fit
                        manager.tx_context.shrink_to_fit().await;
                    }
                    _ => {}
                }
                crate::core::runtime::task::sleep(manager.tx_check_interval).await;
            }
        });
    }

    pub async fn get_mut<'a>(
        &'a self,
        context_id: &str,
    ) -> Option<RefMut<'a, String, (DBTx, TxState)>> {
        self.tx_context.get_mut(context_id).await
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
        //send tx to context
        self.tx_context
            .insert(
                new_context_id.to_string(),
                (conn, TxState::StateBegin(Instant::now())),
            )
            .await;
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
        let tx_op = self.tx_context.remove(context_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!(
                "[rbatis] tx:{} not exist！",
                context_id
            )));
        }
        let (mut tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.commit().await?;
        if self.is_enable_log() {
            self.do_log(context_id, &format!("[rbatis] [{}] Commit", context_id));
        }
        return Ok(context_id.to_string());
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, context_id: &str) -> Result<String, crate::core::Error> {
        let tx_op = self.tx_context.remove(context_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!(
                "[rbatis] tx:{} not exist！",
                context_id
            )));
        }
        let (tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.rollback().await?;
        if self.is_enable_log() {
            self.do_log(context_id, &format!("[rbatis] [{}] Rollback", context_id));
        }
        return Ok(context_id.to_string());
    }

    /// context_id is 'tx:' prifix ?
    pub fn is_tx_prifix_id(&self, context_id: &str) -> bool {
        return context_id.starts_with(&self.tx_prefix);
    }
}

/// the TxGuard just like an  Lock Guard,
/// if TxGuard Drop, this tx will be commit or rollback
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
