#![allow(unsafe_code)]

use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::{Entry, RandomState};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::runtime::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// SyncMap impl the Send and Sync
/// it use of RwLock,so it's safe! but we went convert lifetime ,so use some lifetime convert unsafe method(but it is safe)
#[derive(Debug)]
pub struct SyncMap<K, V> where K: Eq + Hash {
    pub shard: RwLock<HashMap<K, V, RandomState>>,
}

impl<'a, K: 'a + Eq + Hash, V: 'a> SyncMap<K, V> where K: Eq + Hash {
    pub fn new() -> Self {
        Self {
            shard: RwLock::new(HashMap::new())
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            shard: RwLock::new(HashMap::with_capacity(capacity))
        }
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let mut w = self.shard.write().await;
        w.insert(key, value)
    }

    pub async fn remove<Q>(&self, key: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq + ?Sized {
        let mut w = self.shard.write().await;
        w.remove(key)
    }

    pub async fn clear(&self) {
        let mut w = self.shard.write().await;
        w.clear();
    }

    pub async fn shrink_to_fit(&self) {
        let mut w = self.shard.write().await;
        w.shrink_to_fit();
    }

    pub async fn reserve(&self, additional: usize) {
        let mut w = self.shard.write().await;
        w.reserve(additional)
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, HashMap<K, V, RandomState>> {
        self.shard.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, HashMap<K, V, RandomState>> {
        self.shard.write().await
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

    pub async fn get_mut<Q>(&'a self, k: &Q) -> Option<RefMut<'a, K, V>>
        where K: Borrow<Q>,
              Q: Hash + Eq + ?Sized {
        let mut get_ref = RefMut::new(self.shard.write().await, None);
        unsafe {
            let v = get_ref._guard.get_mut(k);
            if v.is_some() {
                let vptr = change_lifetime_mut(v.unwrap());
                get_ref.v = Option::Some(vptr);
                Some(get_ref)
            } else {
                None
            }
        }
    }
}

///this is safe,only change lifetime
pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

///this is safe,only change lifetime
pub unsafe fn change_lifetime_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}

#[derive(Debug)]
pub struct Ref<'a, K, V>
    where K: Eq + Hash {
    _guard: RwLockReadGuard<'a, HashMap<K, V, RandomState>>,
    v: Option<&'a V>,
}

impl<'a, K, V> Ref<'a, K, V> where K: Eq + Hash {
    pub fn new(guard: RwLockReadGuard<'a, HashMap<K, V, RandomState>>, v: Option<&'a V>) -> Self {
        let s = Self {
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

#[derive(Debug)]
pub struct RefMut<'a, K, V, S = RandomState> {
    _guard: RwLockWriteGuard<'a, HashMap<K, V, S>>,
    v: Option<&'a mut V>,
}

impl<'a, K: Eq + Hash, V> RefMut<'a, K, V> {
    pub fn new(guard: RwLockWriteGuard<'a, HashMap<K, V, RandomState>>, v: Option<&'a mut V>) -> Self {
        let s = Self {
            _guard: guard,
            v: v,
        };
        s
    }

    pub fn value(&self) -> &V {
        self.v.as_ref().unwrap()
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.v.as_mut().unwrap()
    }
}


impl<'a, K: Eq + Hash, V> Deref for RefMut<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V> DerefMut for RefMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::sync::Arc;
    use std::time::Duration;
    use std::time::Instant;

    use crate::sync::sync_map::SyncMap;

    #[test]
    fn test_map() {
        let m = Arc::new(SyncMap::new());
        async_std::task::block_on(async {
            m.insert(1, "default".to_string()).await;
            let r = m.get(&1).await;
            let rv = r.unwrap().v;
            println!("r:{:?}", &rv);
            assert_eq!("default", format!("{}", &rv.unwrap()));

            drop(rv);

            let mut mut_v = m.get_mut(&1).await.unwrap();
            *mut_v = "changed".to_string();
            drop(mut_v);
            let r = m.get(&1).await;
            println!("r:{:?}", &r.as_ref().unwrap().deref());
            assert_eq!("changed", format!("{}", &r.as_ref().unwrap().deref()));
        });
    }

    #[test]
    fn test_map_for() {
        let m = Arc::new(SyncMap::new());
        async_std::task::block_on(async {
            let mut lock = m.write().await;
            lock.insert(1, 1);
            drop(lock);
            let lock = m.read().await;
            for (k, v) in lock.deref() {
                println!("k:{},v:{}", k, v);
            }
        });
    }


    //bench on windows10 40 nano/op.  It depends on the runtime(tokio/async_std) speed
    //test command:
    //cargo test --release --color=always --package rbatis-core --lib sync::sync_map::test::bench_test --no-fail-fast -- --exact -Z unstable-options --format=json --show-output
    #[test]
    fn bench_test() {
        let m = Arc::new(SyncMap::new());
        async_std::task::block_on(async {
            let s = m.insert(1, "default".to_string()).await;
            drop(s);

            let total = 100000;
            let now = Instant::now();
            for current in 0..total {
                m.get(&1).await;
                if current == total - 1 {
                    time(total, now.elapsed());
                    break;
                }
            }
            m.shrink_to_fit().await;
            println!("done");
        });
    }

    fn time(total: u64, time: Duration) {
        println!("use Time: {:?} ,each:{} ns/op", &time, time.as_nanos() / (total as u128));
    }
}
