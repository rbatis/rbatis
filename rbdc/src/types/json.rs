use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::Error;
use serde::Deserializer;
use rbs::value::map::ValueMap;

#[derive(serde::Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "Json")]
pub struct Json(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Json::from(Value::deserialize(deserializer)?))
    }
}

impl Default for Json {
    fn default() -> Self {
        Self {
            0: serde_json::Value::Null,
        }
    }
}

impl From<serde_json::Value> for Json {
    fn from(arg: serde_json::Value) -> Self {
        Json(arg)
    }
}

impl From<Value> for Json {
    fn from(v: Value) -> Self {
        match v {
            Value::Null => Json(serde_json::Value::Null),
            Value::Bool(v) => Json(serde_json::Value::Bool(v)),
            Value::I32(v) => Json(serde_json::json!(v)),
            Value::I64(v) => Json(serde_json::json!(v)),
            Value::U32(v) => Json(serde_json::json!(v)),
            Value::U64(v) => Json(serde_json::json!(v)),
            Value::F32(v) => Json(serde_json::json!(v)),
            Value::F64(v) => Json(serde_json::json!(v)),
            Value::String( v) => {
                Json(serde_json::json!(v))
            }
            Value::Binary(v) => Json(serde_json::json!(v)),
            Value::Array(v) => Json(serde_json::json!(v)),
            Value::Map(v) => Json(serde_json::json!(v)),
        }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Json({})", self.0)
    }
}

impl Debug for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Json({})", self.0)
    }
}

impl From<Json> for Value {
    fn from(arg: Json) -> Self {
        match arg.0{
            serde_json::Value::Null => {
                Value::Null
            }
            serde_json::Value::Bool(v) => {
                Value::Bool(v)
            }
            serde_json::Value::Number(v) => {
                if v.is_f64(){
                    Value::F64(v.as_f64().unwrap_or_default())
                }else  if v.is_i64(){
                    Value::I64(v.as_i64().unwrap_or_default())
                }else{
                    Value::U64(v.as_u64().unwrap_or_default())
                }
            }
            serde_json::Value::String(v) => {
                Value::String(v)
            }
            serde_json::Value::Array(v) => {
                let mut arr = Vec::with_capacity(v.capacity());
                for x in v {
                    arr.push(Value::from(Json(x)));
                }
                Value::Array(arr)
            }
            serde_json:: Value::Object(v) => {
                let mut arr = ValueMap::with_capacity(v.len());
                for (k,v) in v {
                    arr.push((Value::String(k),Value::from(Json(v))));
                }
                Value::Map(arr)
            }
        }
    }
}

impl FromStr for Json {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(serde_json::Value::String(s.to_string())))
    }
}

#[cfg(test)]
mod test {
    use crate::json::Json;
    use rbs::value::map::ValueMap;

    #[test]
    fn test_decode_js_string() {
        let m = rbs::Value::String(r#""aa""#.to_string());
        println!("{}", m);
        assert_eq!(r#""aa""#, Json::from(m).0);
    }

    #[test]
    fn test_decode_js_string_map() {
        let mut m = ValueMap::new();
        m.insert("a".into(), "1".into());
        let m = rbs::Value::Map(m);
        println!("{}", m.to_string());
        assert_eq!(r#"{"a":"1"}"#, Json::from(m).0);
    }

    #[test]
    fn test_decode_js_int_map() {
        let mut m = ValueMap::new();
        m.insert("a".into(), 1.into());
        let m = rbs::Value::Map(m);
        println!("{}", m.to_string());
        assert_eq!(r#"{"a":1}"#, Json::from(m).0);
    }

    #[test]
    fn test_decode_js_int_arr() {
        let arr = rbs::Value::Array(vec![rbs::Value::I64(1), rbs::Value::I64(2)]);
        println!("{}", arr.to_string());
        assert_eq!(r#"[1,2]"#, Json::from(arr).0);
    }

    #[test]
    fn test_decode_js_string_arr() {
        let arr = rbs::Value::Array(vec![
            rbs::Value::String(1.to_string()),
            rbs::Value::String(2.to_string()),
        ]);
        println!("{}", arr.to_string());
        assert_eq!(r#"["1","2"]"#, Json::from(arr).0);
    }
}
