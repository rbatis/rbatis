use crate::crud::CRUDTable;
use std::any::Any;

#[async_trait::async_trait]
pub trait TableSync {
    async fn table_sync<T>(&self, t: T)
    where
        T: CRUDTable;
}

pub struct RbatisTableSync {
    pub dbs: Vec<Box<dyn Any>>,
}

impl RbatisTableSync {
    pub fn do_table_sync(&self) {}
}

#[cfg(test)]
mod test {
    use crate::crud::CRUDTable;
    use crate::table_sync::{RbatisTableSync, TableSync};

    #[test]
    fn test_sync_table() {
        let mut s = RbatisTableSync { dbs: vec![] };

        pub struct A {}
        #[async_trait::async_trait]
        impl TableSync for A {
            async fn table_sync<T>(&self, t: T)
            where
                T: CRUDTable,
            {
            }
        }
        s.dbs.push(Box::new(A {}));
    }
}
