
use std::collections::LinkedList;

use crate::tx::propagation::Propagation;
use crate::tx::tx::Tx;

//事务栈
pub struct TxStack {
    len: usize,
    txs: LinkedList<Tx>,
    propagations: LinkedList<Propagation>,
}

impl  TxStack  {
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

    pub fn pop(&mut self) -> (Option<Tx>, Option<Propagation>) {
        if self.len==0{
            return (None,None);
        }
        self.len -= 1;
        return (self.txs.pop_back(), self.propagations.pop_back());
    }

    pub fn first(&mut self) -> (Option<&mut Tx>, Option<&mut Propagation>) {
        return (self.txs.front_mut(), self.propagations.front_mut());
    }
    pub fn first_ref(&self) -> (Option<&Tx>, Option<&Propagation>) {
        return (self.txs.front(), self.propagations.front());
    }

    pub fn last(&mut self) -> (Option<&mut Tx>, Option<&mut Propagation>) {
        return (self.txs.back_mut(), self.propagations.back_mut());
    }
    pub fn last_ref(&self) -> (Option<&Tx>, Option<&Propagation>) {
        return (self.txs.back(), self.propagations.back());
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn have_tx(&self) -> bool {
        return self.len > 0;
    }
}