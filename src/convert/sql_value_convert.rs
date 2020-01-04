use serde_json::{json, Value};

pub trait SqlValueConvert {
    fn to_sql(&self)->String;
}

impl SqlValueConvert for serde_json::Value{
    fn to_sql(&self)->String{
        match self {
            Value::Null => return String::from("null"),
            Value::String(s) => {
                let mut ns=s.clone();
                ns.insert_str(0,"'");
                ns=ns+"'";
                return ns;
            },
            Value::Number(n) => return n.to_string(),
            Value::Bool(b) => return b.to_string(),
            Value::Object(o) => panic!("[rbatis] not support convert Object/Map<String,Value>!"),
            Value::Array(arr) => {
                let mut item="(".to_string();
                for x in arr{
                    match x {
                        serde_json::Value::String(_)=>{
                            item=item+x.to_sql().as_str()+","
                        },
                        serde_json::Value::Number(_)=>{
                            item=item+x.to_sql().as_str()+","
                        }
                        _ => {}
                    }
                }
                item.pop();
                item=item+")";
                return item;
            },
            _ => return String::from(""),
        }
    }
}





#[test]
fn test_convert(){
    let mut result;
    result =   json!(1).to_sql();
    println!("number(i64)=>{}",result);
    result =  json!(1.2).to_sql();
    println!("number(f64)=>{}",result);
    result =  json!("abc").to_sql();
    println!("string=>{}",result);
    result = json!(null).to_sql();
    println!("null=>{}",result);
}
