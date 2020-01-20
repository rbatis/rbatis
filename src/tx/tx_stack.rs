use std::collections::LinkedList;

use crate::tx::propagation::Propagation;
use crate::tx::tx::TxImpl;
use serde::{Deserialize, Serialize};

///事务栈
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxStack {
    len: usize,
    txs: LinkedList<TxImpl>,
    propagations: LinkedList<Propagation>,
}

impl TxStack {
    pub fn new() -> Self {
        return Self {
            len: 0,
            txs: LinkedList::new(),
            propagations: LinkedList::new(),
        };
    }
    pub fn push(&mut self, tx: TxImpl, p: Propagation) {
        self.txs.push_back(tx);
        self.propagations.push_back(p);
        self.len += 1;
    }

    pub fn pop(&mut self) -> (Option<TxImpl>, Option<Propagation>) {
        if self.len == 0 {
            return (None, None);
        }
        self.len -= 1;
        return (self.txs.pop_back(), self.propagations.pop_back());
    }

    pub fn first_pop(&mut self) -> (Option<TxImpl>, Option<Propagation>) {
        return (self.txs.pop_front(), self.propagations.pop_front());
    }

    pub fn first_ref(&self) -> (Option<&TxImpl>, Option<&Propagation>) {
        return (self.txs.front(), self.propagations.front());
    }

    pub fn last_pop(&mut self) -> (Option<TxImpl>, Option<Propagation>) {
        return (self.txs.pop_back(), self.propagations.pop_back());
    }

    pub fn last_ref(&self) -> (Option<&TxImpl>, Option<&Propagation>) {
        return (self.txs.back(), self.propagations.back());
    }

    pub fn close(&mut self) {
        for x in &mut self.txs {
            x.is_close = true;
        }
    }


    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn have_tx(&self) -> bool {
        return self.len > 0;
    }
}

impl Drop for TxStack {
    fn drop(&mut self) {
        self.close();
    }
}