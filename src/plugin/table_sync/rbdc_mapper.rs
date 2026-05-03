use crate::{table_sync::ColumnMapper, RBatis};
use rbs::Value;

impl ColumnMapper for RBatis {
    fn driver_type(&self) -> String {
        self.driver_type().unwrap_or_default().to_string()
    }

    fn get_column_type(&self, column: &str, v: &Value) -> String {
        let pool = self.get_pool().expect("uninit pool");
        let driver = pool.driver();
        let column_type = driver.column_type(v);
        let driver_name = driver.name();
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else {
                    if (driver_name == "mysql" || driver_name == "postgres" || driver_name == "pg")
                        && (column.eq("id") || column.ends_with("_id") || column.starts_with("id_"))
                    {
                        return "VARCHAR(50)".to_string();
                    }
                    "TEXT".to_string()
                }
            }
            _ => column_type,
        }
    }
}
