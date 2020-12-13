use serde_json::Value;

pub mod limit;

pub trait PageLimit {
    /// return  sql
    fn page_limit_sql(&self, offset: u64, size: u64) -> crate::core::Result<String>;
}