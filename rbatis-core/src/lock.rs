use async_std::sync::{RwLock, RwLockReadGuard};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::hash::Hash;


pub struct SyncMap<K, V,S=RandomState>{
    shard: RwLock<HashMap<K, V,S>>,
}


#[cfg(test)]
mod test{
    use crate::lock::SyncMap;

    #[test]
    fn test_map(){
         async_std::task::block_on(async {

         });
    }
}