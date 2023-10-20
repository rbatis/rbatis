use chrono::{FixedOffset, NaiveDateTime};
use fastdate::offset_sec;
use rbdc::datetime::DateTime;
use rbdc::Error;
use rbs::{to_value, Value};
use tiberius::numeric::BigDecimal;
use tiberius::ColumnData;

pub trait Decode {
    fn decode(row: &ColumnData<'static>) -> Result<Value, Error>;
}

impl Decode for Value {
    fn decode(row: &ColumnData<'static>) -> Result<Value, Error> {
        Ok(match row {
            ColumnData::U8(v) => match v {
                None => Value::Null,
                Some(v) => Value::U32(v.clone() as u32),
            },
            ColumnData::I16(v) => match v {
                None => Value::Null,
                Some(v) => Value::I32(v.clone() as i32),
            },
            ColumnData::I32(v) => match v {
                None => Value::Null,
                Some(v) => Value::I32(v.clone()),
            },
            ColumnData::I64(v) => match v {
                None => Value::Null,
                Some(v) => Value::I64(v.clone()),
            },
            ColumnData::F32(v) => match v {
                None => Value::Null,
                Some(v) => Value::F32(v.clone()),
            },
            ColumnData::F64(v) => match v {
                None => Value::Null,
                Some(v) => Value::F64(v.clone()),
            },
            ColumnData::Bit(v) => match v {
                None => Value::Null,
                Some(v) => Value::Bool(v.clone()),
            },
            ColumnData::String(v) => match v {
                None => Value::Null,
                Some(v) => Value::String(v.to_string()),
            },
            ColumnData::Guid(v) => match v {
                None => Value::Null,
                Some(v) => Value::String(v.to_string()).into_ext("Uuid"),
            },
            ColumnData::Binary(v) => match v {
                None => Value::Null,
                Some(v) => Value::Binary(v.to_vec()),
            },
            ColumnData::Numeric(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<BigDecimal>> = tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => Value::String(v.to_string()).into_ext("Decimal"),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::Xml(v) => match v {
                None => Value::Null,
                Some(v) => Value::String(v.to_string()).into_ext("Xml"),
            },
            ColumnData::DateTime(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<NaiveDateTime>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => to_value!(DateTime(fastdate::DateTime::from_timestamp_nano(
                                v.timestamp_nanos_opt()
                                    .expect("value can not be represented in a timestamp with nanosecond precision.") as i128
                                -offset_sec() as i128,
                            ).set_offset(offset_sec()))),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::SmallDateTime(m) => match m {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<NaiveDateTime>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => to_value!(DateTime(fastdate::DateTime::from_timestamp_nano(
                                v.timestamp_nanos_opt()
                                    .expect("value can not be represented in a timestamp with nanosecond precision.") as i128 -offset_sec() as i128,
                            ).set_offset(offset_sec()))),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::Time(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<chrono::NaiveTime>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => Value::String(v.to_string()).into_ext("Time"),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::Date(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<chrono::NaiveDate>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => Value::String(v.to_string()).into_ext("Date"),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::DateTime2(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<NaiveDateTime>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => to_value!(DateTime(fastdate::DateTime::from_timestamp_nano(
                                v.timestamp_nanos_opt()
                                    .expect("value can not be represented in a timestamp with nanosecond precision.") as i128 -offset_sec() as i128,
                            ).set_offset(offset_sec()))),
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
            ColumnData::DateTimeOffset(v) => match v {
                None => Value::Null,
                Some(_) => {
                    let v: tiberius::Result<Option<chrono::DateTime<FixedOffset>>> =
                        tiberius::FromSql::from_sql(row);
                    match v {
                        Ok(v) => match v {
                            None => Value::Null,
                            Some(v) => {
                                let dt = DateTime(fastdate::DateTime::from_timestamp_nano(
                                    v.timestamp_nanos_opt()
                                        .expect("value can not be represented in a timestamp with nanosecond precision.") as i128 - (v.offset().utc_minus_local() * 60) as i128).set_offset(v.offset().utc_minus_local() * 60));
                                to_value!(dt)
                            }
                        },
                        Err(e) => {
                            return Err(Error::from(e.to_string()));
                        }
                    }
                }
            },
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use chrono::{FixedOffset, NaiveDateTime};
    use fastdate::offset_sec;


    #[test]
    fn test_decode_zone() {
        let mut dt = fastdate::DateTime::from_str("2023-12-12T00:00:00Z").unwrap();
        println!("{}", dt.to_string());
        dt = dt.add_sub_sec(-offset_sec() as i64).set_offset(offset_sec());
        println!("{}", dt.to_string());
    }

    #[test]
    fn test_decode_time_zone() {
        let offset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        let dt: chrono::DateTime<FixedOffset> = chrono::DateTime::from_local(NaiveDateTime::from_timestamp_opt(1697801035, 0).unwrap(), offset);
        let date = fastdate::DateTime::from_timestamp_nano(
            dt.timestamp_nanos_opt()
                .expect("value can not be represented in a timestamp with nanosecond precision.") as i128)
            .set_offset(offset_sec());
        println!("{},{}", dt.to_string(), date.to_string());
        assert_eq!(dt.to_string().replacen(" ", "T", 1).replace(" ", ""), date.display(true));
    }
}