
pub mod limit;



pub trait PageLimit {
    fn page_limit_sql(&self, offset:u64, size:u64) -> rbatis_core::Result<String>;
}