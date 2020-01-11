
use std::collections::LinkedList;

use crate::tx::propagation::Propagation;
use crate::tx::tx::Tx;

//事务栈
pub struct TxStack {
    len: usize,
    txs: LinkedList<Tx>,
    propagations: LinkedList<Propagation>,
}

impl TxStack {
    pub fn new()->Self{
        return Self{
            len: 0,
            txs: LinkedList::new(),
            propagations: LinkedList::new()
        }
    }
    pub fn push(&mut self, tx: Tx, p: Propagation) {
        self.txs.push_back(tx);
        self.propagations.push_back(p);
        self.len += 1;
    }

    pub fn pop(&mut self, p: Propagation) -> (Option<Tx>, Option<Propagation>) {
        if self.len==0{
            return (None,None);
        }
        self.len -= 1;
        return (self.txs.pop_back(), self.propagations.pop_back());
    }

    pub fn first(&self) -> (Option<&Tx>, Option<&Propagation>) {
        return (self.txs.front(), self.propagations.front());
    }
    pub fn last(&self) -> (Option<&Tx>, Option<&Propagation>) {
        return (self.txs.back(), self.propagations.back());
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn have_tx(&self) -> bool {
        return self.len > 0;
    }
}