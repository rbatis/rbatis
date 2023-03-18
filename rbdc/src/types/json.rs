use crate::{Error};
use rbs::{to_value, Value};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq)]
pub struct Json(pub serde_json::Value);

impl serde::Serialize for Json{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Json::from(serde_json::Value::deserialize(deserializer)?))
    }
}

impl Default for Json {
    fn default() -> Self {
        Self::from(serde_json::Value::Null)
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
            Value::Null => Json::from(serde_json::Value::Null),
            Value::Bool(v) => Json::from(serde_json::json!(v)),
            Value::I32(v) => Json::from(serde_json::json!(v)),
            Value::I64(v) => Json::from(serde_json::json!(v)),
            Value::U32(v) => Json::from(serde_json::json!(v)),
            Value::U64(v) => Json::from(serde_json::json!(v)),
            Value::F32(v) => Json::from(serde_json::json!(v)),
            Value::F64(v) => Json::from(serde_json::json!(v)),
            Value::String(v) => Json::from(serde_json::json!(v)),
            Value::Binary(v) => Json::from(serde_json::json!(v)),
            Value::Array(v) => Json::from({
                let mut datas = Vec::<serde_json::Value>::with_capacity(v.len());
                for x in v {
                    datas.push(Json::from(x).0);
                }
                serde_json::Value::Array(datas)
            }),
            Value::Map(m) => Json::from({
                let mut datas = serde_json::Map::with_capacity(m.len());
                for (k, v) in m {
                    datas.insert(
                        k.as_str().unwrap_or_default().to_string(),
                        Json::from(v).0,
                    );
                }
                serde_json::Value::Object(datas)
            }),
        }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Json> for Value {
    fn from(arg: Json) -> Self {
        to_value!(arg.0)
    }
}

impl FromStr for Json {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Json::from(
            serde_json::Value::from_str(s).map_err(|e| Self::Err::from(e.to_string()))?,
        ))
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
        assert_eq!(r#"{"a":"1"}"#, Json::from(m).0.to_string());
    }

    #[test]
    fn test_decode_js_int_map() {
        let mut m = ValueMap::new();
        m.insert("a".into(), 1.into());
        let m = rbs::Value::Map(m);
        println!("{}", m.to_string());
        assert_eq!(r#"{"a":1}"#, Json::from(m).0.to_string());
    }

    #[test]
    fn test_decode_js_int_arr() {
        let arr = rbs::Value::Array(vec![rbs::Value::I64(1), rbs::Value::I64(2)]);
        println!("{}", arr.to_string());
        assert_eq!(r#"[1,2]"#, Json::from(arr).0.to_string());
    }

    #[test]
    fn test_decode_js_string_arr() {
        let arr = rbs::Value::Array(vec![
            rbs::Value::String(1.to_string()),
            rbs::Value::String(2.to_string()),
        ]);
        println!("{}", arr.to_string());
        assert_eq!(r#"["1","2"]"#, Json::from(arr).0.to_string());
    }
}
