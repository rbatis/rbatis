use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use serde::Deserializer;
use crate::Error;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Json")]
pub struct Json(pub String);


impl<'de> serde::Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Json::from(Value::deserialize(deserializer)?))
    }
}

impl Default for Json {
    fn default() -> Self {
        Self {
            0: "null".to_string(),
        }
    }
}

impl From<serde_json::Value> for Json {
    fn from(arg: serde_json::Value) -> Self {
        Json(arg.to_string())
    }
}

impl From<Value> for Json {
    fn from(v: Value) -> Self {
        match v {
            Value::Null => {
                Json(v.to_string())
            }
            Value::Bool(v) => {
                Json(v.to_string())
            }
            Value::I32(v) => {
                Json(v.to_string())
            }
            Value::I64(v) => {
                Json(v.to_string())
            }
            Value::U32(v) => {
                Json(v.to_string())
            }
            Value::U64(v) => {
                Json(v.to_string())
            }
            Value::F32(v) => {
                Json(v.to_string())
            }
            Value::F64(v) => {
                Json(v.to_string())
            }
            Value::String(mut v) => {
                if v.starts_with("{") || v.starts_with("[") {
                    Json(v)
                } else {
                    v.insert(0, '"');
                    v.push('"');
                    Json(v)
                }
            }
            Value::Binary(v) => {
                Json(unsafe { String::from_utf8_unchecked(v) })
            }
            Value::Array(_) => {
                Json(v.to_string())
            }
            Value::Map(v) => {
                Json(v.to_string())
            }
            Value::Ext(_name, v) => {
                Json::from(*v)
            }
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
        Value::Ext("Json", Box::new(Value::String(arg.0)))
    }
}

impl FromStr for Json {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}


#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use crate::json::Json;

    #[test]
    fn test_decode_js_string_map() {
        let mut m = ValueMap::new();
        m.insert("a".into(), "1".into());
        let m=rbs::Value::Map(m);
        println!("{}",m.to_string());
        assert_eq!(r#"{"a":"1"}"#, Json::from(m).0);
    }

    #[test]
    fn test_decode_js_int_map() {
        let mut m = ValueMap::new();
        m.insert("a".into(), 1.into());
        let m=rbs::Value::Map(m);
        println!("{}",m.to_string());
        assert_eq!(r#"{"a":1}"#, Json::from(m).0);
    }

    #[test]
    fn test_decode_js_int_arr() {
        let arr = rbs::Value::Array(vec![rbs::Value::I64(1),rbs::Value::I64(2)]);
        println!("{}",arr.to_string());
        assert_eq!(r#"[1,2]"#, Json::from(arr).0);
    }

    #[test]
    fn test_decode_js_string_arr() {
        let arr = rbs::Value::Array(vec![rbs::Value::String(1.to_string()),rbs::Value::String(2.to_string())]);
        println!("{}",arr.to_string());
        assert_eq!(r#"["1","2"]"#, Json::from(arr).0);
    }
}
