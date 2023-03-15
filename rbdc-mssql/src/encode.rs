use std::ops::Index;
use rbdc::Error;
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
            Value::String(v) => {
                q.bind(v);
                Ok(())
            }
            Value::Binary(v) => {
                q.bind(v);
                Ok(())
            }
            Value::Array(_) => Err(Error::from("unimplemented")),
            Value::Map(m) => {
                let v = m.index("value");
                let t = m.index("type").as_str().unwrap_or_default();
                if t != "" {
                    match t {
                        "Date" => {
                            q.bind(
                                chrono::NaiveDate::from_str(v.as_str().unwrap_or_default())
                                    .map_err(|e| Error::from(e.to_string()))?,
                            );
                            Ok(())
                        }
                        "DateTime" => {
                            let mut s = v.as_str().unwrap_or_default().to_string();
                            if s.len() > 10 {
                                s.replace_range(10..11, "T");
                            }
                            q.bind(
                                chrono::NaiveDateTime::from_str(&s)
                                    .map_err(|e| Error::from(e.to_string()))?,
                            );
                            Ok(())
                        }
                        "Time" => {
                            q.bind(
                                chrono::NaiveTime::from_str(v.as_str().unwrap_or_default())
                                    .map_err(|e| Error::from(e.to_string()))?,
                            );
                            Ok(())
                        }
                        "Decimal" => {
                            q.bind(
                                BigDecimal::from_str(&v.clone().into_string().unwrap_or_default())
                                    .map_err(|e| Error::from(e.to_string()))?,
                            );
                            Ok(())
                        }
                        "Json" => Err(Error::from("unimplemented")),
                        "Timestamp" => {
                            q.bind(v.as_u64().unwrap_or_default() as i64);
                            Ok(())
                        }
                        "Uuid" => {
                            q.bind(
                                Uuid::from_str(&v.clone().into_string().unwrap_or_default()).unwrap_or_default(),
                            );
                            Ok(())
                        }
                        _ => {
                            Err(Error::from("unimplemented"))
                        }
                    }
                }else{
                    Err(Error::from("unimplemented"))
                }
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
