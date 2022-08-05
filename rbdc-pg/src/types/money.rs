use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use rbs::Value;
use std::fmt::{Display, Formatter};

/// The raw integer value sent over the wire; for locales with `frac_digits=2` (i.e. most
/// of them), this will be the value in whole cents.
///
/// E.g. for `select '$123.45'::money` with a locale of `en_US` (`frac_digits=2`),
/// this will be `12345`.
///
/// If the currency of your locale does not have fractional units, e.g. Yen, then this will
/// just be the units of the currency.
///
/// See the type-level docs for an explanation of `locale_frac_units`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Money")]
pub struct Money(pub i64);

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Money({})", self.0)
    }
}

impl From<Money> for Value {
    fn from(arg: Money) -> Self {
        Value::Ext("Money", Box::new(Value::I64(arg.0)))
    }
}

impl Encode for Money {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.0.to_be_bytes());
        Ok(IsNull::No)
    }
}

impl Decode for Money {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self({
            match value.format() {
                PgValueFormat::Binary => {
                    let cents = BigEndian::read_i64(value.as_bytes()?);
                    Ok(cents)
                }
                PgValueFormat::Text => Err(Error::from(
                    "Reading a `MONEY` value in text format is not supported.",
                )),
            }
        }?))
    }
}
