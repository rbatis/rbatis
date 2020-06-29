
pub mod limit;



pub trait PageLimit {
    fn create(&self,offset:i64, size:i64) -> rbatis_core::Result<String>;
}