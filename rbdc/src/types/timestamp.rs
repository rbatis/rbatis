use crate::Error;
use rbs::Value;
use serde::Deserializer;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Timestamp(timestamp_millis:u64)
#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Timestamp")]
pub struct Timestamp(pub u64);

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)?.as_u64() {
            None => Err(D::Error::custom("warn type decode Timestamp")),
            Some(v) => Ok(Self(v)),
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
        Value::from(("Timestamp",Value::U64(arg.0)))
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Timestamp(u64::from_str(s)?))
    }
}

#[cfg(test)]
mod test {
    use crate::timestamp::Timestamp;
    use rbs::Value;

    #[test]
    fn test_decode_timestamp_u64() {
        assert_eq!(Timestamp(1), rbs::from_value(Value::U64(1)).unwrap());
    }

    #[test]
    fn test_decode_timestamp_ext() {
        assert_eq!(
            Timestamp(1),
            rbs::from_value(Value::from(("Timestamp",Value::U64(1)))).unwrap()
        );
    }
}
