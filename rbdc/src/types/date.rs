use crate::{Error};
use rbs::{to_value, Value};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Date(pub fastdate::Date);

impl Deref for Date {
    type Target = fastdate::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.0.to_string();
            s.push_str("D");
            serializer.serialize_str(&s)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        if std::any::type_name::<D>() == std::any::type_name::<rbs::Serializer>() {
            use serde::de::Error;
            let mut value = Value::deserialize(deserializer)?;
            match &mut value {
                Value::String(v) => {
                    if v.ends_with("D"){
                        v.pop();
                    }
                }
                _ => {}
            }
            let time: fastdate::Date = rbs::from_value(value)
                .map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
            Ok(Date::from(time))
        } else {
            use serde::de::Error;
            let time: fastdate::Date = rbs::from_value(Value::deserialize(deserializer)?)
                .map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
            Ok(Date::from(time))
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Date> for Value {
    fn from(arg: Date) -> Self {
        Value::Map(rbs::value::map::ValueMap {
            inner: vec![("type".into(), "Date".into()), ("value".into(), to_value!(arg.0))],
        })
    }
}

impl From<fastdate::Date> for Date {
    fn from(arg: fastdate::Date) -> Self {
        Date(arg)
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date::from(fastdate::Date::from_str(s)?))
    }
}

#[cfg(test)]
mod test {
    use rbs::{from_value, to_value};
    use crate::date::Date;

    #[test]
    fn test_date() {
        let date = Date(fastdate::Date {
            day: 1,
            mon: 1,
            year: 2021,
        });
        let d = to_value!(date);
        println!("{}", d);
        let v: Date = from_value(d).unwrap();
        println!("{}", v);
    }
}