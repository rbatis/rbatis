use crate::table_sync::{ColumMapper};
use rbs::Value;

pub struct SqliteTableMapper {}

impl ColumMapper for SqliteTableMapper {
    fn get_column(&self, _column: &str, v: &Value) -> String {
        match v {
            Value::Null => "NULL".to_string(),
            Value::Bool(_) => "BOOLEAN".to_string(),
            Value::I32(_) => "INTEGER".to_string(),
            Value::I64(_) => "INT8".to_string(),
            Value::U32(_) => "INTEGER".to_string(),
            Value::U64(_) => "INT8".to_string(),
            Value::F32(_) => "DOUBLE".to_string(),
            Value::F64(_) => "DOUBLE".to_string(),
            Value::String(v) => {
                if v != "" {
                    v.to_string()
                } else {
                    "TEXT".to_string()
                }
            },
            Value::Binary(_) => "BLOB".to_string(),
            Value::Array(_) => "BLOB".to_string(),
            Value::Map(_) => "BLOB".to_string(),
            Value::Ext(t, _v) => match *t {
                "Date" => "TEXT".to_string(),
                "DateTime" => "TEXT".to_string(),
                "Time" => "TEXT".to_string(),
                "Timestamp" => "INT8".to_string(),
                "Decimal" => "NUMERIC".to_string(),
                "Json" => "BLOB".to_string(),
                "Uuid" => "TEXT".to_string(),
                _ => "NULL".to_string(),
            },
        }
    }
}

