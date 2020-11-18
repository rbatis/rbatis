use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use rbatis_core::db_adapter::DBPool;

use crate::core::db_adapter::DBTx;
use crate::core::sync::sync_map::SyncMap;
use crate::rbatis::Rbatis;

pub struct TxManager {
    pub tx_context: SyncMap<String, (DBTx, TxState)>,
    pub tx_out_of_time: Duration,
    pub check_interval: Duration,
}


pub enum TxState {
    StateBegin(Instant),
    StateRun(Instant),
    StateFinish(Instant),
}


impl TxManager {
    pub fn new() -> Self {
        Self {
            tx_context: SyncMap::new(),
            tx_out_of_time: Duration::from_secs(60),
            check_interval: Duration::from_secs(5),
        }
    }

    pub async fn spawn_check(&self) {
        loop {
            let m = self.tx_context.read().await;
            let mut need_rollback = None;
            for (k, (tx, state)) in m.deref() {
                match state {
                    TxState::StateBegin(instant) => {
                        let out_time = instant.elapsed();
                        if out_time > self.tx_out_of_time {
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
                    let mut m = self.tx_context.write().await;
                    for tx_id in v {
                        m.remove(tx_id);
                    }
                }
                _ => {}
            }
            crate::core::runtime::sleep(self.check_interval);
        }
    }

    /// begin tx,for new conn
    pub async fn begin(&self, new_tx_id: &str, pool: &DBPool) -> Result<(), crate::core::Error> {
        if new_tx_id.is_empty() {
            return Err(crate::core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn: DBTx = pool.begin().await?;
        //send tx to context
        self.tx_context.insert(new_tx_id.to_string(), (conn, TxState::StateBegin(Instant::now()))).await;
        return Ok(());
    }

    /// commit tx,and return conn
    pub async fn commit(&self, tx_id: &str) -> Result<(), crate::core::Error> {
        let tx_op = self.tx_context.remove(tx_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (mut tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.commit().await?;
        return Ok(result);
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, tx_id: &str) -> Result<(), crate::core::Error> {
        let tx_op = self.tx_context.remove(tx_id).await;
        if tx_op.is_none() {
            return Err(crate::core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (tx, state): (DBTx, TxState) = tx_op.unwrap();
        let result = tx.rollback().await?;
        return Ok(result);
    }
}