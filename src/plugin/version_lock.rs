use serde_json::Number;
use std::fmt::Debug;

pub trait VersionLockPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_allow(&self, context_id: &str) -> bool;

    /// database column must be i32 or i64 or time column!
    fn column(&self) -> &str;

    /// set value = value + 1, support number and string value
    fn try_add_one(
        &self,
        context_id: &str,
        old_value: &serde_json::Value,
        column: &str,
    ) -> serde_json::Value {
        if !self.is_allow(context_id) {
            return old_value.clone();
        }
        if self.column().eq(column) {
            match old_value {
                serde_json::Value::String(s) => {
                    let version = s.parse::<i64>();
                    match version {
                        Ok(version) => {
                            return serde_json::Value::String((version + 1).to_string());
                        }
                        _ => {}
                    }
                }
                serde_json::Value::Number(n) => {
                    if n.is_i64() {
                        return serde_json::json!(n.as_i64().unwrap_or(0) + 1);
                    } else if n.is_u64() {
                        return serde_json::json!(n.as_u64().unwrap_or(0) + 1);
                    }
                }
                _ => {}
            }
        }
        return old_value.clone();
    }

    fn try_make_where_sql(&self, context_id: &str, old_version: &serde_json::Value) -> String {
        if !self.is_allow(context_id) {
            return String::default();
        }
        if !old_version.eq(&serde_json::Value::Null) {
            format!("{} = {} ", self.column(), old_version)
        } else {
            return String::default();
        }
    }
}

#[derive(Debug, Clone)]
pub struct RbatisVersionLockPlugin {
    pub excludes: Vec<String>,
    pub version_column: String,
}

impl RbatisVersionLockPlugin {
    pub fn new(version_column: &str) -> Self {
        Self {
            excludes: vec![],
            version_column: version_column.to_owned(),
        }
    }
}

impl VersionLockPlugin for RbatisVersionLockPlugin {
    fn is_allow(&self, context_id: &str) -> bool {
        for x in &self.excludes {
            if context_id.starts_with(x) {
                return false;
            }
        }
        return true;
    }

    fn column(&self) -> &str {
        &self.version_column
    }
}
