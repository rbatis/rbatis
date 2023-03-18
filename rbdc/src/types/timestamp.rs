use crate::{Error, RBDCString};
use rbs::{to_value, Value};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Timestamp(timestamp_millis:u64)
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Timestamp(pub u64);

impl RBDCString for Timestamp {
    fn ends_name() -> &'static str {
        "TS"
    }

    fn decode_str(arg: &str) -> Result<Self, crate::Error> {
        let is = Self::is(arg);
        if is != "" {
            return Ok(Self::from_str(arg.trim_end_matches(Self::ends_name()))?);
        }
        Err(crate::Error::E(format!("warn type decode :{}",Self::ends_name())))
    }
}

impl serde::Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.0.to_string();
            s.push_str(Self::ends_name());
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
                    Timestamp::trim_ends_match(v);
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
    use rbs::Value;

    #[test]
    fn test_decode_timestamp_u64() {
        assert_eq!(Timestamp::from(1), rbs::from_value(Value::U64(1)).unwrap());
    }

    #[test]
    fn test_decode_timestamp_ext() {
        assert_eq!(
            Timestamp::from(1),
            rbs::from_value(Value::String("1".to_string())).unwrap()
        );
    }
}
