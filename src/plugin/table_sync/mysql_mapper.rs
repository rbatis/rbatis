use crate::table_sync::{ColumMapper};
use rbs::Value;

pub struct MysqlTableMapper {}

impl Default for MysqlTableMapper{
    fn default() -> Self {
        MysqlTableMapper{}
    }
}

impl ColumMapper for MysqlTableMapper {
    fn get_column(&self, column: &str, v: &Value) -> &'static str {
        match v {
            Value::Null => "NULL",
            Value::Bool(_) => "TINYINT",
            Value::I32(_) => "INT",
            Value::I64(_) => "BIGINT",
            Value::U32(_) => "INT",
            Value::U64(_) => "BIGINT",
            Value::F32(_) => "FLOAT",
            Value::F64(_) => "DOUBLE",
            Value::String(_) => {
                if column.to_lowercase().contains("id") {
                    "VARCHAR(50)"
                } else {
                    "TEXT"
                }
            }
            Value::Binary(_) => "BLOB",
            Value::Array(_) => "JSON",
            Value::Map(_) => "JSON",
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE",
                "DateTime" => "DATETIME",
                "Time" => "TIME",
                "Timestamp" => "TIMESTAMP",
                "Decimal" => "DECIMAL",
                "Json" => "JSON",
                "Uuid" => "TEXT",
                _ => "NULL",
            },
        }
    }
}

