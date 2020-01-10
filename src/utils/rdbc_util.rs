use serde_json::Value;

pub fn to_rdbc_values(arg_array:&mut Vec<Value>)->Vec<rdbc::Value>{
    let mut params =vec![];
    for x in arg_array {
        match x{
            serde_json::Value::String(v)=>{
                params.push(rdbc::Value::String(v.clone()));
            }
            serde_json::Value::Number(v)=>{
                if v.is_i64(){
                    params.push(rdbc::Value::String(v.as_i64().unwrap().to_string()));
                }else if v.is_u64(){
                    params.push(rdbc::Value::String(v.as_u64().unwrap().to_string()));
                }else if v.is_f64(){
                    params.push(rdbc::Value::String(v.as_f64().unwrap().to_string()));
                }
            }
            _ => {
            }
        }
    }
    return params;
}

pub fn to_string(arg:&Vec<rdbc::Value>)->String{
    let mut s = String::new();
    for x in arg {
        s = s + x.to_string().as_str() + ",";
    }
    if s.len() > 0 {
        s.pop();
    }
    s = "{".to_string()+s.as_str();
    s = s + "}";
    return s;
}