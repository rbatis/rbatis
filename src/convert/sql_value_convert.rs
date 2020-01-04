use serde_json::{json, Value};

const AND: &'static str = " and ";

///转换器，serde_json 的json值转换为 sql 兼容的值
/// 例如转换以下内容
///
///    use serde_json::{json, Value};
///
///    assert_eq!("true".to_string(),json!(true).to_sql());
///    assert_eq!("1".to_string(),json!(1).to_sql());
///    assert_eq!("1.2".to_string(),json!(1.2).to_sql());
///    assert_eq!("'abc'".to_string(),json!("abc").to_sql());
///    assert_eq!("('1','2','3')".to_string(),json!(vec!["1","2","3"]).to_sql());
///    assert_eq!("(1,2,3)".to_string(),json!(vec![1,2,3]).to_sql());
///    assert_eq!("a = 1 and b = 'b' and c = 1.1".to_string(),json!({
///      "a":1,
///      "b":"b",
///      "c":1.1,
///    }).to_sql());
///    assert_eq!("null".to_string(),json!(null).to_sql());
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
            Value::Object(arg_map) => {
                let mut where_str="".to_string();
                let len=arg_map.len();
                for (key,value) in arg_map{
                    match value{
                        Value::String(s)=>{
                            where_str=where_str+key.as_str()+" = "+value.to_sql().as_str() + AND
                        }
                        Value::Number(n)=>{
                            where_str=where_str+key.as_str()+" = "+value.to_sql().as_str() + AND
                        }
                        Value::Array(arr)=>{
                            where_str=where_str+key.as_str()+" in "+value.to_sql().as_str() + AND
                        }
                        _ => {
                        }
                    }
                }
                if len>0{
                    for _ in 0..AND.len() {
                        where_str.pop();
                    }
                }
                return where_str;
            },
            Value::Array(arr) => {
                let len=arr.len();
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
                if len>0{
                    item.pop();
                }
                item=item+")";
                return item;
            },
            _ => return String::from(""),
        }
    }
}





#[test]
fn test_convert(){
    assert_eq!("true".to_string(),json!(true).to_sql());
    assert_eq!("1".to_string(),json!(1).to_sql());
    assert_eq!("1.2".to_string(),json!(1.2).to_sql());
    assert_eq!("'abc'".to_string(),json!("abc").to_sql());
    assert_eq!("('1','2','3')".to_string(),json!(vec!["1","2","3"]).to_sql());
    assert_eq!("(1,2,3)".to_string(),json!(vec![1,2,3]).to_sql());
    assert_eq!("a = 1 and b = 'b' and c = 1.1".to_string(),json!({
      "a":1,
      "b":"b",
      "c":1.1,
    }).to_sql());
    assert_eq!("null".to_string(),json!(null).to_sql());
}
