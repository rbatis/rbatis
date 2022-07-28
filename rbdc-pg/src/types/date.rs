use std::str::FromStr;
use std::time::Duration;
use rbdc::date::Date;
use rbdc::Error;
use crate::value::{PgValue, PgValueFormat};

use super::decode::{Decode, self};


impl Decode for fastdate::Date{
    fn decode(value: PgValue) -> Result<Self, Error> {
       Ok(match value.format() {
           PgValueFormat::Binary => {
               // DATE is encoded as the days since epoch
               let days: i32 = Decode::decode(value)?;
               let dt=fastdate::DateTime{
                   micro: 0,
                   sec: 0,
                   min: 0,
                   hour: 0,
                   year:2000,
                   day:1,
                   mon:1
               } + Duration::from_secs((days * 24 * 3600) as u64);
               fastdate::Date::from(dt)
           }

           PgValueFormat::Text => {
               let dt= fastdate::DateTime::from_str(&format!("{}T00:00:00",value.as_str()?))?;
               fastdate::Date::from(dt)
           },
       })
    }
}

impl Decode for Date{
    fn decode(value: crate::value::PgValue) -> Result<Self, rbdc::Error> {
        todo!()
    }
}

