use crate::table_sync::{ColumMapper};
use rbs::Value;

pub struct PGTableMapper {}
impl ColumMapper for PGTableMapper {
    fn get_column(&self, _column:&str,  v: &Value) -> &'static str {
        match v {
            Value::Null => "NULL",
            Value::Bool(_) => "BOOLEAN",
            Value::I32(_) => "INTEGER",
            Value::I64(_) => "BIGINT",
            Value::U32(_) => "INTEGER",
            Value::U64(_) => "BIGINT",
            Value::F32(_) => "REAL",
            Value::F64(_) => "DOUBLE PRECISION",
            Value::String(_) => {
                "TEXT"
            },
            Value::Binary(_) => "BYTEA",
            Value::Array(_) => "JSON",
            Value::Map(_) => "JSON",
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE",
                "DateTime" => "TIMESTAMP",
                "Time" => "TIME",
                "Timestamp" => "TIMESTAMP",
                "Decimal" => "NUMERIC",
                "Json" => "JSON",
                "Uuid" => "UUID",
                _ => "NULL",
            },
        }
    }
}

