use std::collections::HashMap;
use crate::runtime::Mutex;
use crate::runtime::MutexGuard;
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use crate::Error;

/// sync map is safe for sync and send
///
/// you can use for lazy static
/// lazy_static! {
///   static ref SS:SyncMap<String>= SyncMap::<String>::new();
/// }
///
///
/// SS.put("1", "fuck you".to_string()).await;
/// println!("{:?}", SS.pop("1").await);
///
///
///
#[derive(Debug)]
pub struct SyncMap<T> {
    pub cell: Mutex<DashMap<String, T>>
}


impl<T> SyncMap<T> {
    pub fn new() -> SyncMap<T> {
        SyncMap {
            cell: Mutex::new(DashMap::new())
        }
    }

    /// put an value,this value will move lifetime into SyncMap
    pub async fn put(&self, key: &str, value: T) {
        self.cell.lock().await.insert(key.to_string(), value);
    }

    /// pop value,lifetime will move to caller
    pub async fn pop(&self, key: &str) -> Option<T> {
        let data: Option<(String, T)> = self.cell.lock().await.remove(key);
        if data.is_none() {
            return None;
        } else {
            return Some(data.unwrap().1);
        }
    }

    pub async fn lock<'a>(&'a self) -> MutexGuard<'a, DashMap<String, T>> {
        let data = self.cell.lock().await;
        return data;
    }

}