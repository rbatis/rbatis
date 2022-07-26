use std::fmt::{Display, Formatter};
use byteorder::{ByteOrder, LittleEndian};
use rbdc::Error;
use crate::result_set::MySqlTypeInfo;
use crate::types::{Decode, Encode};
use crate::value::{MySqlValue, MySqlValueFormat};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Year")]
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year({})", self.0)
    }
}

impl Encode for Year{
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.push(2);
        buf.extend_from_slice(&self.0.to_le_bytes());
        Ok(2)
    }
}

impl Decode for Year{
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self({
            match value.format() {
                MySqlValueFormat::Text => value.as_str()?.parse().unwrap_or_default(),
                MySqlValueFormat::Binary => {
                    let buf = value.as_bytes()?;
                    let len = buf[0];
                    LittleEndian::read_u16(&buf[1..])
                }
            }
        }))
    }
}