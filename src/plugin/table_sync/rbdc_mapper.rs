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
        match v {
            Value::String(v) => {
                if !v.is_empty() {
                    v.to_string()
                } else if column.eq("id") || column.ends_with("_id") || column.starts_with("id_") {
                    "VARCHAR(50)".to_string()
                } else {
                    column_type
                }
            }
            _ => column_type,
        }
    }
}
