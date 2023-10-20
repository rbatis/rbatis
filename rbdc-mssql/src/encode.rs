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
            Value::Map(_) => Err(Error::from("unimplemented")),
            Value::Ext(t, v) => match t {
                "Date" => {
                    q.bind(
                        chrono::NaiveDate::from_str(v.as_str().unwrap_or_default())
                            .map_err(|e| Error::from(e.to_string()))?,
                    );
                    Ok(())
                }
                "DateTime" => {
                    let date = fastdate::DateTime::from_str(&v.as_str().unwrap_or_default())?;
                    q.bind(
                        chrono::NaiveDateTime::from_str(&date.display(false))
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
                        BigDecimal::from_str(&v.into_string().unwrap_or_default())
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
                        Uuid::from_str(&v.into_string().unwrap_or_default()).unwrap_or_default(),
                    );
                    Ok(())
                }
                _ => Err(Error::from("unimplemented")),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn test_from() {
        let mut v = fastdate::DateTime::now();
        println!("{}", v.display(false));
        let n = chrono::NaiveDateTime::from_str(&v.display(false)).unwrap();
        assert_eq!(v.display(false),n.to_string().replace(" ","T").trim_end_matches("0"));
    }

}
