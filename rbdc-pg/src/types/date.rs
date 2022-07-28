use std::str::FromStr;
use std::time::Duration;
use rbdc::date::Date;
use rbdc::Error;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};




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

impl Encode for fastdate::Date{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        todo!()
    }
}

impl Decode for Date{
    fn decode(value: PgValue) -> Result<Self, Error> {
        todo!()
    }
}

impl Encode for Date{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        todo!()
    }
}

