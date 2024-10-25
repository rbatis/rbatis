use crate::table_sync::ColumnMapper;
use rbs::Value;

pub struct PGTableMapper {}
impl ColumnMapper for PGTableMapper {
    fn driver_type(&self) -> String {
        "postgres".to_string()
    }
    fn get_column_type(&self, _column: &str, v: &Value) -> String {
        match v {
            Value::Null => "NULL".to_string(),
            Value::Bool(_) => "BOOLEAN".to_string(),
            Value::I32(_) => "INTEGER".to_string(),
            Value::I64(_) => "BIGINT".to_string(),
            Value::U32(_) => "INTEGER".to_string(),
            Value::U64(_) => "BIGINT".to_string(),
            Value::F32(_) => "REAL".to_string(),
            Value::F64(_) => "DOUBLE PRECISION".to_string(),
            Value::String(v) => {
                if v != "" {
                    if v.eq("id") {
                        return "TEXT".to_string();
                    }
                    v.to_string()
                } else {
                    "TEXT".to_string()
                }
            }
            Value::Binary(_) => "BYTEA".to_string(),
            Value::Array(_) => "JSON".to_string(),
            Value::Map(_) => "JSON".to_string(),
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE".to_string(),
                "DateTime" => "TIMESTAMPTZ".to_string(),
                "Time" => "TIME".to_string(),
                "Timestamp" => "TIMESTAMP".to_string(),
                "Decimal" => "NUMERIC".to_string(),
                "Json" => "JSON".to_string(),
                "Uuid" => "UUID".to_string(),
                _ => "NULL".to_string(),
            },
        }
    }
}
