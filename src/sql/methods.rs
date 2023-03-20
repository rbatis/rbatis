use rbatis_codegen::ops::AsProxy;
use rbdc::common::time::Time;
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::RBDCString;
use rbdc::timestamp::Timestamp;
use rbdc::uuid::Uuid;
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
            v => {
                if v.is_str() {
                    let v = v.str();
                    let mut sql = String::new();
                    if Date::is(&v) != "" {
                        sql.push_str("'");
                        sql.push_str(v.trim_end_matches(Date::ends_name()));
                        sql.push_str("'");
                    } else if DateTime::is(&v) != "" {
                        sql.push_str("'");
                        sql.push_str(v.trim_end_matches(DateTime::ends_name()));
                        sql.push_str("'");
                    } else if Time::is(&v) != "" {
                        sql.push_str("'");
                        sql.push_str(v.trim_end_matches(Time::ends_name()));
                        sql.push_str("'");
                    } else if Timestamp::is(&v) != "" {
                        //timestamp is u64 type,do not add ''
                        sql.push_str(v.trim_end_matches(Timestamp::ends_name()));
                    } else if Decimal::is(&v) != "" {
                        sql.push_str("'");
                        sql.push_str(v.trim_end_matches(Decimal::ends_name()));
                        sql.push_str("'");
                    } else if Uuid::is(&v) != "" {
                        sql.push_str("'");
                        sql.push_str(v.trim_end_matches(Uuid::ends_name()));
                        sql.push_str("'");
                    } else {
                        sql.push_str("'");
                        sql.push_str(v);
                        sql.push_str("'");
                    }
                    Value::String(sql)
                } else {
                    Value::String(v.string_sql())
                }
            }
        }
    }
}
