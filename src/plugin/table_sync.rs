use std::any::Any;

#[async_trait::async_trait]
pub trait TableSync {
    async fn table_sync<T>(&self, t: T);
}

pub struct RbatisTableSync {
    pub dbs: Vec<Box<dyn Any>>,
}

impl RbatisTableSync {
    pub fn do_table_sync(&self) {}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sync_table() {

    }
}
