use std::str::FromStr;
use chrono::{FixedOffset, NaiveDateTime, Utc};
use rbs::Value;
use tiberius::numeric::BigDecimal;
use tiberius::{ColumnData};
use rbdc::datetime::FastDateTime;
use rbdc::Error;

pub trait Decode {
    fn decode(row: &ColumnData<'static>) -> Result<Value, Error>;
}

impl Decode for Value {
    fn decode(row: &ColumnData<'static>) -> Result<Value, Error> {
        Ok(match row {
            ColumnData::U8(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::U32(v.clone() as u32)
                    }
                }
            }
            ColumnData::I16(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::I32(v.clone() as i32)
                    }
                }
            }
            ColumnData::I32(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::I32(v.clone())
                    }
                }
            }
            ColumnData::I64(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::I64(v.clone())
                    }
                }
            }
            ColumnData::F32(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::F32(v.clone())
                    }
                }
            }
            ColumnData::F64(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::F64(v.clone())
                    }
                }
            }
            ColumnData::Bit(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::Bool(v.clone())
                    }
                }
            }
            ColumnData::String(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::String(v.to_string())
                    }
                }
            }
            ColumnData::Guid(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::String(v.to_string()).into_ext("Uuid")
                    }
                }
            }
            ColumnData::Binary(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::Binary(v.to_vec())
                    }
                }
            }
            ColumnData::Numeric(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<BigDecimal>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::String(v.to_string()).into_ext("Decimal")
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::Xml(v) => {
                match v {
                    None => { Value::Null }
                    Some(v) => {
                        Value::String(v.to_string()).into_ext("Xml")
                    }
                }
            }
            ColumnData::DateTime(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<NaiveDateTime>> = tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                       Value::from(FastDateTime::from_str(&v.to_string())?)
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::SmallDateTime(m) => {
                match m {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<chrono::NaiveDateTime>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::from(FastDateTime::from_str(&v.to_string())?)
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::Time(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<chrono::NaiveTime>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::String(v.to_string()).into_ext("Time")
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::Date(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<chrono::NaiveDate>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::String(v.to_string()).into_ext("Date")
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::DateTime2(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<chrono::DateTime<Utc>>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::from(FastDateTime::from_str(&v.to_string())?)
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
            ColumnData::DateTimeOffset(v) => {
                match v {
                    None => { Value::Null }
                    Some(_) => {
                        let v:tiberius::Result<Option<chrono::DateTime<FixedOffset>>>=tiberius::FromSql::from_sql(row);
                        match v{
                            Ok(v) => {
                                match v{
                                    None => {Value::Null}
                                    Some(v) => {
                                        Value::from(FastDateTime::from_str(&v.to_string())?)
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(Error::from(e.to_string()));
                            }
                        }
                    }
                }
            }
        })
    }
}
