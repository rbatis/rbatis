use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc};
use std::collections::HashMap;
use crate::runtime::Mutex;


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
    pub cell: Mutex<RefCell<HashMap<String, T>>>
}

impl<T> SyncMap<T> {
    pub fn new() -> SyncMap<T> {
        SyncMap {
            cell: Mutex::new(RefCell::new(HashMap::new()))
        }
    }

    /// put an value,this value will move lifetime into SyncMap
    pub async fn put(&self, key: &str, value: T) {
        let lock = self.cell.lock().await;
        let mut b = lock.borrow_mut();
        b.insert(key.to_string(), value);
    }

    /// pop value,lifetime will move to caller
    pub async fn pop(&self, key: &str) -> Option<T> {
        let lock = self.cell.lock().await;
        let mut b = lock.borrow_mut();
        return b.remove(key);
    }
}