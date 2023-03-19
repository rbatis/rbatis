use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::{Error, RBDCString};
use rbs::Value;
use std::str::FromStr;
use tiberius::numeric::BigDecimal;
use tiberius::{Query, Uuid};

pub trait Encode {
    fn encode(self, q: &mut Query) -> Result<(), Error>;
}

impl Encode for Value {
    fn encode(self, q: &mut Query) -> Result<(), Error> {
        match self {
            Value::Null => {
                q.bind(Option::<String>::None);
                Ok(())
            }
            Value::Bool(v) => {
                q.bind(v);
                Ok(())
            }
            Value::I32(v) => {
                q.bind(v);
                Ok(())
            }
            Value::I64(v) => {
                q.bind(v);
                Ok(())
            }
            Value::U32(v) => {
                q.bind(v as i32);
                Ok(())
            }
            Value::U64(v) => {
                q.bind(v as i64);
                Ok(())
            }
            Value::F32(v) => {
                q.bind(v);
                Ok(())
            }
            Value::F64(v) => {
                q.bind(v);
                Ok(())
            }
            Value::String(mut v) => {
                if Date::is(&v) != "" {
                    Date::trim_ends_match(&mut v);
                    q.bind(
                        chrono::NaiveDate::from_str(&v).map_err(|e| Error::from(e.to_string()))?,
                    );
                    Ok(())
                } else if DateTime::is(&v) != "" {
                    DateTime::trim_ends_match(&mut v);
                    if v.len() > 10 {
                        v.replace_range(10..11, "T");
                    }
                    q.bind(
                        chrono::NaiveDateTime::from_str(&v)
                            .map_err(|e| Error::from(e.to_string()))?,
                    );
                    Ok(())
                } else if Time::is(&v) != "" {
                    Time::trim_ends_match(&mut v);
                    q.bind(
                        chrono::NaiveTime::from_str(&v).map_err(|e| Error::from(e.to_string()))?,
                    );
                    Ok(())
                } else if Timestamp::is(&v) != "" {
                    Timestamp::trim_ends_match(&mut v);
                    let ts = Timestamp::decode_str(v.as_str())?;
                    q.bind(ts.0 as i64);
                    Ok(())
                } else if Decimal::is(&v) != "" {
                    Decimal::trim_ends_match(&mut v);
                    q.bind(BigDecimal::from_str(&v).map_err(|e| Error::from(e.to_string()))?);
                    Ok(())
                } else if rbdc::types::uuid::Uuid::is(&v) != "" {
                    rbdc::types::uuid::Uuid::trim_ends_match(&mut v);
                    q.bind(Uuid::from_str(&v).unwrap_or_default());
                    Ok(())
                } else {
                    q.bind(v);
                    Ok(())
                }
            }
            Value::Binary(v) => {
                q.bind(v);
                Ok(())
            }
            Value::Array(arr) => {
                q.bind(Json::from(Value::Array(arr)).0.to_string());
                Ok(())
            }
            Value::Map(m) => {
                q.bind(Json::from(Value::Map(m)).0.to_string());
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn test_from() {
        let mut v = fastdate::DateTime::now().to_string();
        v.replace_range(10..11, "T");
        println!("{}", v.to_string());
        let n = chrono::NaiveDateTime::from_str(&v).unwrap();
        assert_eq!(n.to_string().replace(" ", "T"), v.to_string());
    }
}
