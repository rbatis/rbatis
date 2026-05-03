use crate::ops::AsProxy;
use rbs::Value;
use std::borrow::Cow;

pub trait IntoSql {
    fn sql(&self) -> String;
}

impl IntoSql for bool {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for String {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for &str {
    fn sql(&self) -> String {
        self.to_string()
    }
}
impl IntoSql for i32 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for i64 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for f32 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for f64 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for u32 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for u64 {
    fn sql(&self) -> String {
        self.to_string()
    }
}

impl IntoSql for Value {
    fn sql(&self) -> String {
        match self {
            Value::Map(m) => {
                let mut sql = "".to_string();
                for (k, v) in m {
                    let k_str = k.clone().string();
                    sql.push_str(&k_str);
                    if v.is_str() {
                        sql.push('\'');
                        sql.push_str(&v.string());
                        sql.push('\'');
                        sql.push(' ');
                    } else if v.is_array() {
                        sql.push_str(&v.sql());
                        sql.push(' ');
                    } else {
                        sql.push_str(&v.string());
                        sql.push(' ');
                    }
                }
                sql
            }
            Value::Array(arr) => {
                let mut sql = "(".to_string();
                for x in arr {
                    if x.is_str() {
                        sql.push('\'');
                        sql.push_str(&x.string());
                        sql.push('\'');
                        sql.push(',');
                    } else {
                        sql.push_str(&x.string());
                        sql.push(',');
                    }
                }
                if !arr.is_empty() {
                    sql.pop();
                }
                sql.push(')');
                sql
            }
            x => {
                if x.is_str() {
                    let mut sql = String::new();
                    sql.push('\'');
                    sql.push_str(&x.clone().string());
                    sql.push('\'');
                    sql
                } else {
                    x.string()
                }
            }
        }
    }
}

impl IntoSql for &Value {
    fn sql(&self) -> String {
        match self {
            Value::Map(m) => {
                let mut sql = "".to_string();
                for (k, v) in m {
                    let k_str = k.clone().string();
                    sql.push_str(&k_str);
                    if v.is_str() {
                        sql.push('\'');
                        sql.push_str(&v.string());
                        sql.push('\'');
                        sql.push(' ');
                    } else if v.is_array() {
                        sql.push_str(&v.sql());
                        sql.push(' ');
                    } else {
                        sql.push_str(&v.string());
                        sql.push(' ');
                    }
                }
                sql
            }
            Value::Array(arr) => {
                let mut sql = "(".to_string();
                for x in arr {
                    if x.is_str() {
                        sql.push('\'');
                        sql.push_str(&x.string());
                        sql.push('\'');
                        sql.push(',');
                    } else {
                        sql.push_str(&x.string());
                        sql.push(',');
                    }
                }
                if !arr.is_empty() {
                    sql.pop();
                }
                sql.push(')');
                sql
            }
            x => {
                if x.is_str() {
                    let mut sql = String::new();
                    sql.push('\'');
                    sql.push_str(&(**x).clone().string());
                    sql.push('\'');
                    sql
                } else {
                    x.string()
                }
            }
        }
    }
}

impl IntoSql for Cow<'_, Value> {
    fn sql(&self) -> String {
        self.as_ref().sql()
    }
}
