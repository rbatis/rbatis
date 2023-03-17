use rbs::value::map::ValueMap;
use rbs::Value;
use std::ops::Index;

pub mod bytes;
///this mod support Some common types, the actual type may need to use the type provided by the driver
///
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod json;
pub mod time;
pub mod timestamp;
pub mod uuid;

pub trait IntoValue {
    fn value(&self) -> &rbs::Value;
    fn into_value(self) -> rbs::Value;
}

impl IntoValue for rbs::Value {
    fn value(&self) -> &Value {
        match self {
            rbs::Value::Null => self,
            rbs::Value::Bool(_) => self,
            rbs::Value::I32(_) => self,
            rbs::Value::I64(_) => self,
            rbs::Value::U32(_) => self,
            rbs::Value::U64(_) => self,
            rbs::Value::F32(_) => self,
            rbs::Value::F64(_) => self,
            rbs::Value::String(_) => self,
            rbs::Value::Binary(_) => self,
            rbs::Value::Array(_) => self,
            rbs::Value::Map(m) => {
                if m.len() == 2 && is_type_value(&m) {
                    m.index("value")
                } else {
                    self
                }
            }
        }
    }

    fn into_value(self) -> rbs::Value {
        match self {
            rbs::Value::Null => self,
            rbs::Value::Bool(_) => self,
            rbs::Value::I32(_) => self,
            rbs::Value::I64(_) => self,
            rbs::Value::U32(_) => self,
            rbs::Value::U64(_) => self,
            rbs::Value::F32(_) => self,
            rbs::Value::F64(_) => self,
            rbs::Value::String(_) => self,
            rbs::Value::Binary(_) => self,
            rbs::Value::Array(_) => self,
            rbs::Value::Map(mut m) => {
                if m.len() == 2 && is_type_value(&m) {
                    m.rm("value")
                } else {
                    rbs::Value::Map(m)
                }
            }
        }
    }
}

fn is_type_value(m: &ValueMap) -> bool {
    let mut have_type = false;
    let mut have_value = false;
    for (k, _) in m {
        if k == "type" {
            have_type = true
        }
        if k == "value" {
            have_value = true
        }
    }
    have_type && have_value
}

pub struct TV {
    pub r#type: &'static str,
    pub value: Value,
}

impl TV {
    pub fn new(t: &'static str, v: Value) -> Self {
        Self {
            r#type: t,
            value: v,
        }
    }
}

impl From<TV> for Value {
    fn from(value: TV) -> Self {
        let m = ValueMap::from(vec![
            ("type".into(), value.r#type.into()),
            ("value".into(), value.value.into()),
        ]);
        Value::Map(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbs::Value;

    #[test]
    fn test_into_value_with_type_value() {
        let mut m = ValueMap::new();
        m.insert("type".into(), Value::String("test".to_string()));
        m.insert("value".into(), Value::I32(42));
        let v = Value::I32(42);
        assert_eq!(v.clone().into_value(), v);
    }

    #[test]
    fn test_into_value_without_type_value() {
        let v = Value::Null;
        assert_eq!(v.clone().into_value(), v);
    }
}
