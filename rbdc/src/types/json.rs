use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::DeserializeOwned;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Json")]
pub struct Json(pub String);

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
            Value::Null => Json(v.to_string()),
            Value::Bool(v) => Json(v.to_string()),
            Value::I32(v) => Json(v.to_string()),
            Value::I64(v) => Json(v.to_string()),
            Value::U32(v) => Json(v.to_string()),
            Value::U64(v) => Json(v.to_string()),
            Value::F32(v) => Json(v.to_string()),
            Value::F64(v) => Json(v.to_string()),
            Value::String(mut v) => {
                if (v.starts_with("{") && v.ends_with("}"))
                    || (v.starts_with("[") && v.ends_with("]"))
                    || (v.starts_with("\"") && v.ends_with("\""))
                {
                    //is json-string
                    Json(v)
                } else {
                    v.insert(0, '"');
                    v.push('"');
                    Json(v)
                }
            }
            Value::Binary(v) => Json(unsafe { String::from_utf8_unchecked(v) }),
            Value::Array(_) => Json(v.to_string()),
            Value::Map(v) => Json(v.to_string()),
            Value::Ext(_name, v) => Json::from(*v),
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

impl From<Json> for serde_json::Value {
    fn from(arg: Json) -> Self {
        let v: serde_json::Value = serde_json::from_str(&arg.0).unwrap_or_default();
        v
    }
}

impl FromStr for Json {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

/// unstable
/// use:
/// #[derive(Clone, serde::Serialize, serde::Deserialize)]
/// pub struct BizUser {
///     pub id: Option<String>,
///     pub account: Option<JsonV<Account>>,
/// }
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct JsonV<T: Serialize + DeserializeOwned>(pub T);

impl<T: Serialize + DeserializeOwned> Serialize for JsonV<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if std::any::type_name::<S::Error>() == std::any::type_name::<rbs::Error>() {
            Json(serde_json::to_string(&self.0).map_err(|e| {
                Error::custom(e.to_string())
            })?).serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for JsonV<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        use serde::de::Error;
        if std::any::type_name::<D::Error>() == std::any::type_name::<rbs::Error>() {
            let mut v = Value::deserialize(deserializer)?;
            let js;
            if let Value::Ext(_ty, buf) = v {
                v = *buf;
            }
            if let Value::Binary(buf) = v {
                js = String::from_utf8(buf).map_err(|e|D::Error::custom(e.to_string()))?;
            }else if let Value::String(buf) = v {
                js = buf;
            } else {
                js = v.to_string();
            }
            if std::any::type_name::<D::Error>() == std::any::type_name::<rbs::Error>() {
                Ok(JsonV(serde_json::from_str(&js).map_err(|e| {
                    Error::custom(e.to_string())
                })?))
            } else {
                Ok(JsonV(serde_json::from_str(&js).map_err(|e| {
                    Error::custom(e.to_string())
                })?))
            }
        } else {
            Ok(JsonV(T::deserialize(deserializer)?))
        }
    }
}

impl<T: Serialize + DeserializeOwned> Deref for JsonV<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + DeserializeOwned> DerefMut for JsonV<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + DeserializeOwned + Display> Display for JsonV<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
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
