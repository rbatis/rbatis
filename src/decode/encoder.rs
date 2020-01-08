
use rdbc::{DataType, ResultSetMetaData, Error, ResultSet};
use serde_json::{Value, Number};
use rdbc_mysql::MySQLResultSet;


pub fn encode_to_value(arg:&mut ResultSet)->Vec<Value>{
    let mut arr=vec![];
    while arg.next() {
        let mut meta_data =arg.meta_data().unwrap();
        for c_index in 0..meta_data.num_columns(){
            let mut m=serde_json::Map::new();
            let c_name=meta_data.column_name(c_index);
            let c_type=meta_data.column_type(c_index);
            match c_type {
                DataType::Utf8 |DataType::Date| DataType::Time |  DataType::Datetime=>{
                    let strings=arg.get_string(c_index);
                    if strings.is_ok() {
                        let v=strings.unwrap();
                        if v.is_some(){
                            m.insert(c_name,serde_json::Value::String(v.unwrap().clone()));
                        }else{
                            m.insert(c_name,serde_json::Value::Null);
                        }
                    }
                }
                DataType::Integer=>{
                    let strings=arg.get_i64(c_index);
                    if strings.is_ok() {
                        let v=strings.unwrap();
                        if v.is_some(){
                            m.insert(c_name,serde_json::Value::Number(Number::from_f64(v.unwrap() as f64).unwrap()));
                        }else{
                            m.insert(c_name,serde_json::Value::Null);
                        }
                    }
                }
                DataType::Float | DataType::Double | DataType::Decimal=>{
                    let strings=arg.get_f64(c_index);
                    if strings.is_ok() {
                        let v=strings.unwrap();
                        if v.is_some(){
                            m.insert(c_name,serde_json::Value::Number(Number::from_f64(v.unwrap()).unwrap()));
                        }else{
                            m.insert(c_name,serde_json::Value::Null);
                        }
                    }
                }
                _ => {}
            }
            arr.push(serde_json::Value::Object(m));
        }
    }
    return arr;
}