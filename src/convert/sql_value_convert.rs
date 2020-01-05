use serde_json::{json, Value};

pub const AND: &'static str = " and ";

pub const SKIP_TYPE_STRING: &'static str = "string";
pub const SKIP_TYPE_NULL: &'static str = "null";
pub const SKIP_TYPE_BOOL: &'static str = "bool";
pub const SKIP_TYPE_NUMBER: &'static str = "number";
pub const SKIP_TYPE_ARRAY: &'static str = "array";
pub const SKIP_TYPE_OBJECT: &'static str = "object";

///转换器，serde_json 的json值转换为 sql 兼容的值
/// 例如转换以下内容
///
///    use serde_json::{json, Value};
///
///    assert_eq!("true".to_string(),json!(true).to_sql_value());
///    assert_eq!("1".to_string(),json!(1).to_sql_value());
///    assert_eq!("1.2".to_string(),json!(1.2).to_sql_value());
///    assert_eq!("'abc'".to_string(),json!("abc").to_sql_value());
///    assert_eq!("('1','2','3')".to_string(),json!(vec!["1","2","3"]).to_sql_value());
///    assert_eq!("(1,2,3)".to_string(),json!(vec![1,2,3]).to_sql_value());
///    assert_eq!("a = 1 and b = 'b' and c = 1.1".to_string(),json!({
///      "a":1,
///      "b":"b",
///      "c":1.1,
///    }).to_sql());
///    assert_eq!("null".to_string(),json!(null).to_sql_value());
pub trait SqlValueConvert {
    fn to_sql_value(&self) -> String;

    fn to_sql_value_custom(&self, skip_type: &str) -> String;

    fn to_sql_value_custom_separator(&self, skip_type: &str, obj_map_separtor: &str, array_separtor: &str) -> String;
}

/// 转换为sql column 逗号分隔的字符串
pub trait SqlColumnConvert {
    fn to_sql_column(&self) -> String;
}

impl SqlValueConvert for serde_json::Value {
    fn to_sql_value(&self) -> String {
        return self.to_sql_value_custom(SKIP_TYPE_NULL);
    }

    fn to_sql_value_custom_separator(&self,  skip_col_type: &str, obj_map_separtor: &str, array_separtor: &str) -> String {
        match self {
            Value::Null => return String::from("null"),
            Value::String(s) => {
                let mut ns = s.clone();
                ns.insert_str(0, "'");
                ns = ns + "'";
                return ns;
            }
            Value::Number(n) => return n.to_string(),
            Value::Bool(b) => return b.to_string(),
            Value::Object(arg_map) => {
                let mut append = false;
                let mut where_str = "".to_string();
                let len = arg_map.len();
                for (key, value) in arg_map {
                    match value {
                        Value::String(s) => {
                            if skip_col_type.contains(SKIP_TYPE_STRING) {
                                continue;
                            }
                            where_str = where_str + key.as_str() + " = " + value.to_sql_value().as_str() + obj_map_separtor;
                            append = true;
                        }
                        Value::Number(n) => {
                            if skip_col_type.contains(SKIP_TYPE_NUMBER) {
                                continue;
                            }
                            where_str = where_str + key.as_str() + " = " + value.to_sql_value().as_str() + obj_map_separtor;
                            append = true;
                        }
                        Value::Array(arr) => {
                            if skip_col_type.contains(SKIP_TYPE_ARRAY) {
                                continue;
                            }
                            where_str = where_str + key.as_str() + " in " + value.to_sql_value().as_str() + obj_map_separtor;
                            append = true;
                        }
                        Value::Null => {
                            if skip_col_type.contains( SKIP_TYPE_NULL) {
                                continue;
                            }
                            where_str = where_str + key.as_str() + " = " + value.to_sql_value().as_str() + obj_map_separtor;
                            append = true;
                        }
                        _ => {}
                    }
                }
                if append {
                    for _ in 0..obj_map_separtor.len() {
                        where_str.pop();
                    }
                }
                return where_str;
            }
            Value::Array(arr) => {
                let mut append = false;
                let mut item = "(".to_string();
                for x in arr {
                    match x {
                        Value::String(_) => {
                            if skip_col_type.contains(SKIP_TYPE_STRING) {
                                continue;
                            }
                            item = item + x.to_sql_value().as_str() + array_separtor;
                            append = true;
                        }
                        Value::Number(_) => {
                            if skip_col_type.contains(SKIP_TYPE_NUMBER) {
                                continue;
                            }
                            item = item + x.to_sql_value().as_str() + array_separtor;
                            append = true;
                        }
                        Value::Null => {
                            if skip_col_type.contains( SKIP_TYPE_NULL) {
                                continue;
                            }
                            item = item + x.to_sql_value().as_str() + array_separtor;
                            append = true;
                        }
                        _ => {}
                    }
                }
                if append {
                    for _ in 0..array_separtor.len() {
                        item.pop();
                    }
                }
                item = item + ")";
                return item;
            }
            _ => return String::from(""),
        }
    }

    fn to_sql_value_custom(&self, skip_col_type: &str) -> String {
        return self.to_sql_value_custom_separator(skip_col_type, AND, ",");
    }
}


impl SqlColumnConvert for Vec<String> {
    fn to_sql_column(&self) -> String {
        let mut sql = "".to_string();
        let mut append = false;
        for item in self {
            sql = sql + item.as_str() + ",";
            append = true;
        }
        if append {
            sql.pop();
        }
        return sql;
    }
}

impl SqlColumnConvert for Vec<&str> {
    fn to_sql_column(&self) -> String {
        let mut sql = "".to_string();
        let mut append = false;
        for item in self {
            sql = sql + *item + ",";
            append = true;
        }
        if append {
            sql.pop();
        }
        return sql;
    }
}

impl SqlColumnConvert for serde_json::Value {
    fn to_sql_column(&self) -> String {
        let mut sql = "".to_string();
        let mut append = false;
        match self {
            Value::String(s) => {
                sql = sql + s.as_str() + ",";
                append = true;
            }
            Value::Object(obj_map) => {
                for (key, item) in obj_map {
                    sql = sql + key.as_str() + ",";
                    append = true;
                }
            }
            _ => {}
        }
        if append {
            sql.pop();
        }
        return sql;
    }
}


#[test]
fn test_convert() {
    assert_eq!("true".to_string(), json!(true).to_sql_value());
    assert_eq!("1".to_string(), json!(1).to_sql_value());
    assert_eq!("1.2".to_string(), json!(1.2).to_sql_value());
    assert_eq!("'abc'".to_string(), json!("abc").to_sql_value());
    assert_eq!("('1','2','3')".to_string(), json!(vec!["1","2","3"]).to_sql_value());
    assert_eq!("(1,2,3)".to_string(), json!(vec![1,2,3]).to_sql_value());
    assert_eq!("a = 1 and b = 'b' and c = 1.1".to_string(), json!({
      "a":1,
      "b":"b",
      "c":1.1,
    }).to_sql_value());
    assert_eq!("null".to_string(), json!(null).to_sql_value());
}

#[test]
fn test_conver_str_array() {
    let arr = vec!["1", "2", "3"];
    let columns = arr.to_sql_column();
    println!("columns:{}", columns);
}