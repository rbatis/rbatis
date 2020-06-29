
pub mod limit;



pub trait PageLimit {
    fn page_limit_sql(&self, offset:i64, size:i64) -> rbatis_core::Result<String>;
}