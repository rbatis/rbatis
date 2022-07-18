use serde::de::Error;
use serde::ser::SerializeStruct;
use serde::{Deserializer, Serializer};
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Date {
    /// Year: four digits
    pub year: u16,
    /// Month: 1 to 12
    pub month: u8,
    /// Day: 1 to {28, 29, 30, 31} (based on month & year)
    pub day: u8,
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Date {}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if type_name::<S::Error>().eq("rbs::value::ext::Error") {
            let mut s = serializer.serialize_struct("Date", 1)?;
            s.serialize_field(
                "date",
                &format!("{:04}-{:02}-{:02}", self.year, self.month, self.day),
            )?;
            s.end()
        } else {
            serializer.serialize_str(&format!(
                "{:04}-{:02}-{:02}",
                self.year, self.month, self.day
            ))
        }
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Proxy {
            pub date: String,
        }
        if type_name::<D::Error>().eq("rbs::value::ext::Error") {
            let v = Proxy::deserialize(deserializer)?;
            let date =
                speedate::Date::parse_str(&v.date).map_err(|e| D::Error::custom(e.to_owned()))?;
            Ok(Self {
                year: date.year,
                month: date.month,
                day: date.day,
            })
        } else {
            let v = String::deserialize(deserializer)?;
            let r = speedate::Date::parse_str(&v).map_err(|e| D::Error::custom(e.to_owned()))?;
            Ok(Self {
                year: r.year,
                month: r.month,
                day: r.day,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Date;

    #[test]
    fn test_ser() {
        let d = Date {
            year: 1997,
            month: 1,
            day: 2,
        };
        let js = serde_json::to_string(&d).unwrap();
        let v = rbs::to_value(&d).unwrap();
        println!("{}", js);
        println!("{}", v);

        let js_date: Date = serde_json::from_str(&js).unwrap();
        let v_date: Date = rbs::from_value(v).unwrap();
        println!("{}", js_date);
        println!("{}", v_date);
    }
}
