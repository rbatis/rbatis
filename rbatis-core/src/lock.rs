#![allow(unsafe_code)]
use async_std::sync::{RwLock, RwLockReadGuard};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;


pub struct SyncMap<K, V> where K: Eq + Hash {
    shard: RwLock<HashMap<K, V, RandomState>>,
}

impl<'a, K: 'a + Eq + Hash, V: 'a> SyncMap<K, V> where K: Eq + Hash {
    pub fn new() -> Self {
        Self {
            shard: RwLock::new(HashMap::new())
        }
    }

    pub async fn get<Q>(&'a self, k: &Q) -> Ref<'a, K, V>
        where  K: Borrow<Q>,
               Q: Hash + Eq + ?Sized {
        let mut get_ref=Ref::new(self.shard.read().await,None);
        unsafe{
            let v=get_ref._guard.get(k);
            let vptr=change_lifetime_const(v.unwrap());
            get_ref.v= Option::from(vptr);
        }
        get_ref
    }
}

pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

pub struct Ref<'a, K, V>
    where K: Eq + Hash {
    _guard: RwLockReadGuard<'a, HashMap<K, V, RandomState>>,
    v: Option<&'a V>,
}

impl<'a, K, V> Ref<'a, K, V> where K: Eq + Hash {
    pub fn new(guard: RwLockReadGuard<'a,HashMap<K, V, RandomState>>, v:Option<&'a V>) -> Self {
        let mut s = Self {
            _guard: guard,
            v:v,
        };
        s
    }
    pub fn value(&self) -> &V {
        self.v.unwrap()
    }
}

impl<'a, K: Eq + Hash, V> Deref for Ref<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        &self.value()
    }
}


#[cfg(test)]
mod test {
    use crate::lock::SyncMap;
    use std::collections::HashMap;
    use dashmap::lock::RwLock;
    use std::sync::Arc;

    #[test]
    fn test_map() {
        let m = Arc::new(SyncMap::new());
        async_std::task::block_on(async {
            let mut w = m.shard.write().await;
            w.insert(1, "sad".to_string());
            drop(w);
            let r=m.get(&1).await;
            println!("r:{}",&r.v.unwrap());
        });
    }
}