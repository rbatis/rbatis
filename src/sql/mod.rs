use serde_json::Value;

pub mod limit;

pub mod date;


pub trait PageLimit {
    /// return  sql
    fn page_limit_sql(&self, offset:u64, size:u64) -> rbatis_core::Result<String>;
}


pub trait Date {
    /// return  (sql,value)
    fn date_convert(&self,value:&serde_json::Value,index:usize) -> rbatis_core::Result<(String, Value)>;
}