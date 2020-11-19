use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use crate::core::runtime::{Arc, RwLock, RwLockReadGuard};

use rbatis_core::db_adapter::DBPool;

use crate::core::db_adapter::DBTx;
use crate::core::sync::sync_map::{RefMut, SyncMap};
use crate::plugin::log::LogPlugin;
use crate::rbatis::Rbatis;

///the Transaction manager，It manages the life cycle of transactions and provides access across threads
///every tx_check_interval check tx is out of time(tx_out_of_time).if out, rollback tx.
///if tx manager will be drop, manager will rollback all of tx.
pub struct TxManager {
    pub tx_context: SyncMap<String, (DBTx, TxState)>,
    pub tx_out_of_time: Duration,
    pub tx_check_interval: Duration,
    pub alive: RwLock<bool>,
    pub log_plugin: Option<Box<dyn LogPlugin>>,
}

impl Drop for TxManager{
    fn drop(&mut self) {
        if self.is_enable_log() {
            self.do_log("[rbatis] tx_polling_check exit;");
        }
    }
}


pub enum TxState {
    StateBegin(Instant),
    StateFinish(Instant),
}


impl TxManager {
    pub fn new(plugin: Box<dyn LogPlugin>, tx_out_of_time: Duration, check_interval: Duration) -> Self {
        Self {
            tx_context: SyncMap::new(),
            tx_out_of_time,
            tx_check_interval: check_interval,
            alive: RwLock::new(true),
            log_plugin: Some(plugin),
        }
    }

    pub async fn set_alive(&self, alive: bool) {
        let mut l = self.alive.write().await;
        *l = alive;
    }

    pub async fn get_alive(&self) -> RwLockReadGuard<'_, bool> {
        self.alive.read().await
    }


    fn is_enable_log(&self) -> bool {
        self.log_plugin.is_some() && self.log_plugin.as_ref().unwrap().is_enable()
    }

    fn do_log(&self, arg: &str) {
        match &self.log_plugin {
            Some(v) => {
                v.do_log(arg);
            }
            _ => {}
        }
    }

    ///polling check tx alive
    pub fn tx_polling_check(manager: &Arc<TxManager>) {
        let manager = manager.clone();
        crate::core::runtime::spawn(async move {
            loop {
                if manager.get_alive().await.deref() == &false {
                    //rollback all
                    let m = manager.tx_context.read().await;
                    let mut rollback_ids = vec![];
                    for (k, (tx, state)) in m.deref() {
                        rollback_ids.push(k.to_string());
                    }
                    drop(m);
                    for tx_id in &rollback_ids {
                        if manager.is_enable_log() {
                            manager.do_log(&format!("[rbatis] rollback tx_id:{},Because the manager exits", tx_id));
                        }
                        manager.rollback(tx_id).await;
                    }
                    drop(manager);
                    return;
                }
                let m = manager.tx_context.read().await;
                let mut need_rollback = None;
                for (k, (tx, state)) in m.deref() {
                    match state {
                        TxState::StateBegin(instant) => {
                            let out_time = instant.elapsed();
                            if out_time > manager.tx_out_of_time {
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
                        for tx_id in v {
                            if manager.is_enable_log() {
                                manager.do_log(&format!("[rbatis] rollback tx_id:{},out of time:{:?}", tx_id, &manager.tx_out_of_time));
                            }
                            manager.rollback(tx_id).await;
                        }
                        //shrink_to_fit
                        manager.tx_context.shrink_to_fit().await;
                    }
                    _ => {}
                }
                crate::core::runtime::sleep(manager.tx_check_interval).await;
            }
        });
    }


    pub async fn get_mut<'a>(&'a self, tx_id: &str) -> Option<RefMut<'a, String, (DBTx, TxState)>> {
        self.tx_context.get_mut(tx_id).await
    }

    /// begin tx,for new conn
    pub async fn begin(&self, new_tx_id: &str, pool: &DBPool) -> Result<u64, crate::core::Error> {
        if new_tx_id.is_empty() {
            return Err(crate::core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn: DBTx = pool.begin().await?;
        //send tx to context
        self.tx_context.insert(new_tx_id.to_string(), (conn, TxState::StateBegin(Instant::now()))).await;
        return Ok(1);
    }

    /// commit tx,and return conn
    pub async fn commit(&self, tx_id: &str) -> Result<u64, crate::core::Error> {
        let tx_op = self.tx_context.remove(tx_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (mut tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.commit().await?;
        return Ok(1);
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, tx_id: &str) -> Result<u64, crate::core::Error> {
        let tx_op = self.tx_context.remove(tx_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.rollback().await?;
        return Ok(1);
    }
}