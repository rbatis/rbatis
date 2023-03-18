use rbatis_codegen::ops::AsProxy;
use rbs::Value;

pub trait IntoSql {
    fn sql(&self) -> Value;
}

impl IntoSql for Value {
    fn sql(&self) -> Value {
        match self {
            Value::Map(m) => {
                let mut sql = "".to_string();
                for (k, v) in m {
                    let k_str = k.str();
                    sql.push_str(k_str);
                    if v.is_str() {
                        sql.push_str("'");
                        sql.push_str(&v.string_sql());
                        sql.push_str("'");
                        sql.push_str(" ");
                    } else {
                        sql.push_str(&v.string_sql());
                        sql.push_str(" ");
                    }
                }
                Value::String(sql)
            }
            Value::Array(arr) => {
                let mut sql = "(".to_string();
                for x in arr {
                    if x.is_str() {
                        sql.push_str("'");
                        sql.push_str(&x.string_sql());
                        sql.push_str("'");
                        sql.push_str(",");
                    } else {
                        sql.push_str(&x.string_sql());
                        sql.push_str(",");
                    }
                }
                if arr.len() != 0 {
                    sql.pop();
                    sql.push_str(")");
                }
                Value::String(sql)
            }
            x => {
                if x.is_str() {
                    let mut sql = String::new();
                    sql.push_str("'");
                    sql.push_str(x.str());
                    sql.push_str("'");
                    Value::String(sql)
                } else {
                    Value::String(x.string_sql())
                }
            }
        }
    }
}
