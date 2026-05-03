use crate::table_sync::ColumnMapper;
use rbs::Value;

// ---- Deprecated table mapper structs - replaced by RBatis (implements ColumnMapper) ----

#[deprecated(note = "use RBatis directly (it now implements ColumnMapper)")]
pub struct SqliteTableMapper;

#[allow(deprecated)]
impl ColumnMapper for SqliteTableMapper {
    fn driver_type(&self) -> String {
        "sqlite".to_string()
    }
    fn get_column_type(&self, column: &str, v: &Value) -> String {
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                    "VARCHAR(50)".to_string()
                } else {
                    "TEXT".to_string()
                }
            }
            _ => "TEXT".to_string(),
        }
    }
}

#[deprecated(note = "use RBatis directly (it now implements ColumnMapper)")]
pub struct MysqlTableMapper;

#[allow(deprecated)]
impl ColumnMapper for MysqlTableMapper {
    fn driver_type(&self) -> String {
        "mysql".to_string()
    }
    fn get_column_type(&self, column: &str, v: &Value) -> String {
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                    "VARCHAR(50)".to_string()
                } else {
                    "TEXT".to_string()
                }
            }
            _ => "TEXT".to_string(),
        }
    }
}

#[deprecated(note = "use RBatis directly (it now implements ColumnMapper)")]
pub struct MssqlTableMapper;

#[allow(deprecated)]
impl ColumnMapper for MssqlTableMapper {
    fn driver_type(&self) -> String {
        "mssql".to_string()
    }
    fn get_column_type(&self, column: &str, v: &Value) -> String {
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                    "VARCHAR(50)".to_string()
                } else {
                    "NVARCHAR(MAX)".to_string()
                }
            }
            _ => "TEXT".to_string(),
        }
    }
}

#[deprecated(note = "use RBatis directly (it now implements ColumnMapper)")]
pub struct PGTableMapper;

#[allow(deprecated)]
impl ColumnMapper for PGTableMapper {
    fn driver_type(&self) -> String {
        "postgres".to_string()
    }
    fn get_column_type(&self, column: &str, v: &Value) -> String {
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                    "VARCHAR(50)".to_string()
                } else {
                    "TEXT".to_string()
                }
            }
            _ => "TEXT".to_string(),
        }
    }
}
