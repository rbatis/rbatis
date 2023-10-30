use crate::table_sync::{ColumMapper};
use rbs::Value;

pub struct MssqlTableMapper {}
impl ColumMapper for MssqlTableMapper {
    fn get_column(&self, v: &Value) -> &'static str {
        match v {
            Value::Null => "NULL",
            Value::Bool(_) => "BIT",
            Value::I32(_) => "INT",
            Value::I64(_) => "BIGINT",
            Value::U32(_) => "INT",
            Value::U64(_) => "BIGINT",
            Value::F32(_) => "REAL",
            Value::F64(_) => "FLOAT",
            Value::String(_) => "NVARCHAR(MAX)",
            Value::Binary(_) => "VARBINARY(MAX)",
            Value::Array(_) => "NVARCHAR(MAX)", // or appropriate JSON type
            Value::Map(_) => "NVARCHAR(MAX)",   // or appropriate JSON type
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE",
                "DateTime" => "DATETIME2",
                "Time" => "TIME",
                "Timestamp" => "DATETIME2",
                "Decimal" => "DECIMAL",
                "Json" => "NVARCHAR(MAX)", // or appropriate JSON type
                "Uuid" => "NVARCHAR(36)",
                _ => "NULL",
            },
        }
    }
}

