use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Mutex, Arc, RwLock};
use std::collections::HashMap;

/// sync map is safe for sync and send
///
/// you can use for lazy static
/// lazy_static! {
///   static ref SS:SyncMap<String>= SyncMap::<String>::new();
/// }
/// SS.put("1", "fuck you".to_string());
/// println!("{:?}", SS.pop("1"));
///
#[derive(Debug, Clone)]
pub struct SyncMap<T> {
    pub cell: Arc<Mutex<RefCell<HashMap<String, T>>>>
}

impl<T> SyncMap<T> {
    pub fn new() -> SyncMap<T> {
        SyncMap {
            cell: Arc::new(Mutex::new(RefCell::new(HashMap::new())))
        }
    }

    pub fn put(&self, key: &str, value: T) {
        let c = self.cell.clone();
        loop {
            let lock = c.lock();
            if lock.is_ok() {
                let lock = lock.unwrap();
                let mut b = lock.borrow_mut();
                b.insert(key.to_string(), value);
                return;
            }
        }
    }

    pub fn pop(&self, key: &str) -> Option<T> {
        let c = self.cell.clone();
        loop {
            let lock = c.lock();
            if lock.is_ok() {
                let lock = lock.unwrap();
                let mut b = lock.borrow_mut();
                return b.remove(key);
            }
        }
    }
}

#[test]
pub fn test_sync_map() {
    let map = SyncMap::new();
    map.put("1", "fuck you".to_string());
    println!("{:?}", map.pop("1"));
}