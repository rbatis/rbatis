use serde_json::Number;
use std::fmt::Debug;

pub trait VersionLockPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// database column must be i32 or i64 or time column!
    fn column(&self) -> &str;

    /// set value = value - 1, support number and string value
    fn try_reduce_one(&self, source_value: serde_json::Value) -> serde_json::Value {
        match source_value {
            serde_json::Value::String(s) => {
                let version = s.parse::<i64>();
                match version {
                    Ok(version) => {
                        return serde_json::Value::String((version - 1).to_string());
                    }
                    _ => {
                        return serde_json::Value::String(s);
                    }
                }
            }
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    return serde_json::json!(n.as_i64().unwrap_or(0) - 1);
                } else if n.is_u64() {
                    return serde_json::json!(n.as_u64().unwrap_or(0) - 1);
                } else {
                    return serde_json::json!(n);
                }
            }
            _ => {
                return source_value;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RbatisVersionLockPlugin {
    pub version_column: String,
}

impl RbatisVersionLockPlugin {
    pub fn new(version_column: &str) -> Self {
        Self {
            version_column: version_column.to_owned(),
        }
    }
}

impl VersionLockPlugin for RbatisVersionLockPlugin {
    fn column(&self) -> &str {
        &self.version_column
    }
}
