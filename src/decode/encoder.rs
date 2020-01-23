use rdbc::{DataType, ResultSetMetaData, Error, ResultSet};
use serde_json::{Value, Number};
use serde_json::de::ParserNumber;


pub fn encode_to_value(arg: &mut dyn ResultSet) -> Vec<Value> {
    let mut arr = vec![];
    while arg.next() {
        let meta_data = arg.meta_data().unwrap();
        let mut m = serde_json::Map::new();
        for c_index in 0..meta_data.num_columns() {
            let c_name = meta_data.column_name(c_index);
            let c_type = meta_data.column_type(c_index);
            match c_type {

                 DataType::Date | DataType::Time | DataType::Datetime => {
                    let date_bytes= arg.get_date(c_index);
                    if date_bytes.is_ok() {
                        let v = date_bytes.unwrap();
                        if v.is_some() {
                            m.insert(c_name, serde_json::Value::String(v.unwrap()));
                        } else {
                            m.insert(c_name, serde_json::Value::Null);
                        }
                    }
                }

                DataType::Char | DataType::Utf8 => {
                    let strings = arg.get_string(c_index);
                    if strings.is_ok() {
                        let v = strings.unwrap();
                        if v.is_some() {
                            m.insert(c_name, serde_json::Value::String(v.unwrap()));
                        } else {
                            m.insert(c_name, serde_json::Value::Null);
                        }
                    }
                }
                DataType::Integer => {
                    let strings = arg.get_i64(c_index);
                    if strings.is_ok() {
                        let v = strings.unwrap();
                        if v.is_some() {
                            m.insert(c_name, serde_json::Value::Number(Number::from(ParserNumber::I64(v.unwrap()))));
                        } else {
                            m.insert(c_name, serde_json::Value::Null);
                        }
                    }
                }
                DataType::Float | DataType::Double | DataType::Decimal => {
                    let strings = arg.get_f64(c_index);
                    if strings.is_ok() {
                        let v = strings.unwrap();
                        if v.is_some() {
                            m.insert(c_name, serde_json::Value::Number(Number::from_f64(v.unwrap()).unwrap()));
                        } else {
                            m.insert(c_name, serde_json::Value::Null);
                        }
                    }
                }
                DataType::Bool => {
                    let byte_opt = arg.get_bytes(c_index);
                    if byte_opt.is_err() {
                        m.insert(c_name, serde_json::Value::Null);
                    } else {
                        let data = byte_opt.unwrap();
                        if data.is_none() {
                            m.insert(c_name, serde_json::Value::Null);
                        } else {
                            let data_bool_vec = data.unwrap();
                            if data_bool_vec.is_empty() {
                                m.insert(c_name, serde_json::Value::Null);
                            } else {
                                let data_bool = *data_bool_vec.get(0).unwrap();
                                if data_bool == 1 {
                                    m.insert(c_name, serde_json::Value::Bool(true));
                                } else {
                                    m.insert(c_name, serde_json::Value::Bool(false));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        arr.push(serde_json::Value::Object(m));
    }
    return arr;
}