use std::borrow::Cow;
use crate::Error;
use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Date")]
pub struct Date {
    pub r#type: Cow<'static,str>,
    pub value: fastdate::Date,
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Date> for Value {
    fn from(arg: Date) -> Self {
        Value::from(vec![("Date".into(),arg.value.to_string().into())])
    }
}

impl From<fastdate::Date> for Date {
    fn from(arg: fastdate::Date) -> Self {
        Date {
            r#type: Cow::Borrowed("Date"),
            value: arg,
        }
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date::from(fastdate::Date::from_str(s)?))
    }
}
