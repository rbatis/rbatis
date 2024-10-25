use crate::table_sync::ColumnMapper;
use rbs::Value;

pub struct MssqlTableMapper {}
impl ColumnMapper for MssqlTableMapper {
    fn driver_type(&self) -> String {
        "mssql".to_string()
    }

    fn get_column_type(&self, _column: &str, v: &Value) -> String {
        match v {
            Value::Null => "NULL".to_string(),
            Value::Bool(_) => "BIT".to_string(),
            Value::I32(_) => "INT".to_string(),
            Value::I64(_) => "BIGINT".to_string(),
            Value::U32(_) => "INT".to_string(),
            Value::U64(_) => "BIGINT".to_string(),
            Value::F32(_) => "REAL".to_string(),
            Value::F64(_) => "FLOAT".to_string(),
            Value::String(v) => {
                if v != "" {
                    if v.eq("id") {
                        return "NVARCHAR(MAX)".to_string();
                    }
                    v.to_string()
                } else {
                    "NVARCHAR(MAX)".to_string()
                }
            }
            Value::Binary(_) => "VARBINARY(MAX)".to_string(),
            Value::Array(_) => "NVARCHAR(MAX)".to_string(), // or appropriate JSON type
            Value::Map(_) => "NVARCHAR(MAX)".to_string(),   // or appropriate JSON type
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE".to_string(),
                "DateTime" => "DATETIME2".to_string(),
                "Time" => "TIME".to_string(),
                "Timestamp" => "DATETIME2".to_string(),
                "Decimal" => "DECIMAL".to_string(),
                "Json" => "NVARCHAR(MAX)".to_string(), // or appropriate JSON type
                "Uuid" => "NVARCHAR(36)".to_string(),
                _ => "NULL".to_string(),
            },
        }
    }
}
