use crate::table_sync::ColumnMapper;
use rbs::Value;

pub struct MysqlTableMapper {}

impl Default for MysqlTableMapper {
    fn default() -> Self {
        MysqlTableMapper {}
    }
}

impl ColumnMapper for MysqlTableMapper {
    fn driver_type(&self) -> String {
        "mysql".to_string()
    }
    fn get_column_type(&self, column: &str, v: &Value) -> String {
        match v {
            Value::Null => "NULL".to_string(),
            Value::Bool(_) => "TINYINT".to_string(),
            Value::I32(_) => "INT".to_string(),
            Value::I64(_) => "BIGINT".to_string(),
            Value::U32(_) => "INT".to_string(),
            Value::U64(_) => "BIGINT".to_string(),
            Value::F32(_) => "FLOAT".to_string(),
            Value::F64(_) => "DOUBLE".to_string(),
            Value::String(v) => {
                if v != "" {
                    v.to_string()
                } else {
                    if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                        return "VARCHAR(50)".to_string();
                    }
                    "VARCHAR(100)".to_string()
                }
            }
            Value::Binary(_) => "BLOB".to_string(),
            Value::Array(_) => "JSON".to_string(),
            Value::Map(_) => "JSON".to_string(),
            Value::Ext(t, _v) => match *t {
                "Date" => "DATE".to_string(),
                "DateTime" => "DATETIME".to_string(),
                "Time" => "TIME".to_string(),
                "Timestamp" => "TIMESTAMP".to_string(),
                "Decimal" => "DECIMAL".to_string(),
                "Json" => "JSON".to_string(),
                "Uuid" => "VARCHAR(50)".to_string(),
                _ => "NULL".to_string(),
            },
        }
    }
}
