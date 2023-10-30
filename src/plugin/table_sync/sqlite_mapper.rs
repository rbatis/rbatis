use crate::table_sync::{ColumMapper};
use rbs::Value;

pub struct SqliteTableMapper {}
impl ColumMapper for SqliteTableMapper {
    fn get_column(&self, _column:&str, v: &Value) -> &'static str {
        match v {
            Value::Null => "NULL",
            Value::Bool(_) => "BOOLEAN",
            Value::I32(_) => "INTEGER",
            Value::I64(_) => "INT8",
            Value::U32(_) => "INTEGER",
            Value::U64(_) => "INT8",
            Value::F32(_) => "DOUBLE",
            Value::F64(_) => "DOUBLE",
            Value::String(_) => "TEXT",
            Value::Binary(_) => "BLOB",
            Value::Array(_) => "BLOB",
            Value::Map(_) => "BLOB",
            Value::Ext(t, _v) => match *t {
                "Date" => "TEXT",
                "DateTime" => "TEXT",
                "Time" => "TEXT",
                "Timestamp" => "INT8",
                "Decimal" => "NUMERIC",
                "Json" => "BLOB",
                "Uuid" => "TEXT",
                _ => "NULL",
            },
        }
    }
}

