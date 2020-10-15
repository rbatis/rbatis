#![allow(unsafe_code)]
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;
use crate::runtime::{RwLock, RwLockReadGuard};


pub struct SyncMap<K, V> where K: Eq + Hash {
    shard: RwLock<HashMap<K, V, RandomState>>,
}

impl<'a, K: 'a + Eq + Hash, V: 'a> SyncMap<K, V> where K: Eq + Hash {
    pub fn new() -> Self {
        Self {
            shard: RwLock::new(HashMap::new())
        }
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let mut w = self.shard.write().await;
        w.insert(key, value)
    }

    pub async fn get<Q>(&'a self, k: &Q) -> Option<Ref<'a, K, V>>
        where K: Borrow<Q>,
              Q: Hash + Eq + ?Sized {
        let mut get_ref = Ref::new(self.shard.read().await, None);
        unsafe {
            let v = get_ref._guard.get(k);
            if v.is_some() {
                let vptr = change_lifetime_const(v.unwrap());
                get_ref.v = Option::from(vptr);
                Some(get_ref)
            } else {
                None
            }
        }
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
    pub fn new(guard: RwLockReadGuard<'a, HashMap<K, V, RandomState>>, v: Option<&'a V>) -> Self {
        let mut s = Self {
            _guard: guard,
            v: v,
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
            let v = m.insert(1, "sad".to_string()).await;
            let r = m.get(&1).await;
            let rv=r.unwrap().v;
            println!("r:{:?}", &rv);
        });
    }
}