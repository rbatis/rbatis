use crate::{Error, IntoValue};
use rbs::{to_value, Value};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Timestamp(timestamp_millis:u64)
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Timestamp(pub u64);

impl serde::Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.0.to_string();
            s.push_str("TS");
            serializer.serialize_str(&s)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        let mut value = Value::deserialize(deserializer)?;
        match &mut value {
            Value::String(v) => {
                if std::any::type_name::<D>() == std::any::type_name::<rbs::Serializer>() {
                    if v.ends_with("TS") {
                        v.pop();
                        v.pop();
                    }
                    let time: u64 = v.parse().map_err(|e| D::Error::custom(&format!("warn type decode Timestamp:{}", e)))?;
                    Ok(Self::from(time))
                } else {
                    let time: u64 = v.parse().map_err(|e| D::Error::custom(&format!("warn type decode Timestamp:{}", e)))?;
                    Ok(Self::from(time))
                }
            }
            Value::U64(v) => {
                Ok(Self::from(*v))
            }
            _ => {Err(D::Error::custom(&format!("warn type decode Timestamp")))}
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Timestamp> for Value {
    fn from(arg: Timestamp) -> Self {
        to_value!(arg)
    }
}

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        Timestamp(value)
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Timestamp::from(u64::from_str(s)?))
    }
}

#[cfg(test)]
mod test {
    use crate::timestamp::Timestamp;
    use crate::TV;
    use rbs::Value;

    #[test]
    fn test_decode_timestamp_u64() {
        assert_eq!(Timestamp::from(1), rbs::from_value(Value::U64(1)).unwrap());
    }

    #[test]
    fn test_decode_timestamp_ext() {
        assert_eq!(
            Timestamp::from(1),
            rbs::from_value(Value::from(TV::new("Timestamp", Value::U64(1)))).unwrap()
        );
    }
}
