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

    pub async fn put(&self, key: &str, value: T) {
        let c = self.cell.clone();
            let lock = c.lock().await;
                let mut b = lock.borrow_mut();
                b.insert(key.to_string(), value);
    }

    pub async fn pop(&self, key: &str) -> Option<T> {
        let c = self.cell.clone();
         let lock = c.lock().await;
        let mut b = lock.borrow_mut();
         return b.remove(key);

    }
}

#[test]
pub fn test_sync_map() {
    async_std::task::block_on(async move {
        let map = SyncMap::new();
        map.put("1", "fuck you".to_string()).await;
        println!("{:?}", map.pop("1").await);
    });
}