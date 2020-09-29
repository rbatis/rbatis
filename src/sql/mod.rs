use serde_json::Value;

pub mod limit;

pub mod date;


pub trait PageLimit {
    /// return  sql
    fn page_limit_sql(&self, offset: u64, size: u64) -> rbatis_core::Result<String>;
}