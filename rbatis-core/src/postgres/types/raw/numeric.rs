use core::convert::TryInto;

use byteorder::BigEndian;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::io::{Buf, BufMut};
use crate::postgres::protocol::TypeId;
use crate::postgres::{PgData, PgRawBuffer, PgTypeInfo, PgValue, Postgres};
use crate::types::Type;
use crate::Error;

/// Represents a `NUMERIC` value in the **Postgres** wire protocol.
#[derive(Debug, PartialEq, Eq)]
pub enum PgNumeric {
    /// Equivalent to the `'NaN'` value in Postgres. The result of, e.g. `1 / 0`.
    NotANumber,
    /// A populated `NUMERIC` value.
    ///
    /// A description of these fields can be found here (although the type being described is the
    /// version for in-memory calculations, the field names are the same):
    /// https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L224-L269
    Number {
        /// The sign of the value: positive (also set for 0 and -0), or negative.
        sign: PgNumericSign,
        /// The digits of the number in base-10000 with the most significant digit first
        /// (big-endian).
        ///
        /// The length of this vector must not overflow `i16` for the binary protocol.
        ///
        /// *Note*: the `Encode` implementation will panic if any digit is `>= 10000`.
        digits: Vec<i16>,
        /// The scaling factor of the number, such that the value will be interpreted as
        ///
        /// ```text
        ///   digits[0] * 10,000 ^ weight
        /// + digits[1] * 10,000 ^ (weight - 1)
        /// ...
        /// + digits[N] * 10,000 ^ (weight - N) where N = digits.len() - 1
        /// ```
        /// May be negative.
        weight: i16,
        /// How many _decimal_ (base-10) digits following the decimal point to consider in
        /// arithmetic regardless of how many actually follow the decimal point as determined by
        /// `weight`--the comment in the Postgres code linked above recommends using this only for
        /// ignoring unnecessary trailing zeroes (as trimming nonzero digits means reducing the
        /// precision of the value).
        ///
        /// Must be `>= 0`.
        scale: i16,
    },
}

// https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L167-L170
const SIGN_POS: u16 = 0x0000;
const SIGN_NEG: u16 = 0x4000;
const SIGN_NAN: u16 = 0xC000; // overflows i16 (C equivalent truncates from integer literal)

/// Possible sign values for [PgNumeric].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PgNumericSign {
    Positive = SIGN_POS,
    Negative = SIGN_NEG,
}

impl PgNumericSign {
    fn try_from_u16(val: u16) -> crate::Result<Self> {
        match val {
            SIGN_POS => Ok(PgNumericSign::Positive),
            SIGN_NEG => Ok(PgNumericSign::Negative),
            SIGN_NAN => panic!("BUG: sign value for NaN passed to PgNumericSign"),
            _ => Err(Error::Decode(
                format!("invalid value for PgNumericSign: {:#04X}", val).into(),
            )),
        }
    }
}

impl Type<Postgres> for PgNumeric {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::NUMERIC, "NUMERIC")
    }
}
impl PgNumeric {
    pub(crate) fn from_bytes(mut bytes: &[u8]) -> crate::Result<Self> {
        // https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L874
        let num_digits = bytes.get_u16::<BigEndian>()?;
        let weight = bytes.get_i16::<BigEndian>()?;
        let sign = bytes.get_u16::<BigEndian>()?;
        let scale = bytes.get_i16::<BigEndian>()?;
        if sign == SIGN_NAN {
            Ok(PgNumeric::NotANumber)
        } else {
            let digits: Vec<_> = (0..num_digits)
                .map(|_| bytes.get_i16::<BigEndian>())
                .collect::<Result<_, _>>()?;
            Ok(PgNumeric::Number {
                sign: PgNumericSign::try_from_u16(sign)?,
                scale,
                weight,
                digits,
            })
        }
    }
}
/// ### Note
///
/// Receiving `PgNumeric` is currently only supported for the Postgres
/// binary (prepared statements) protocol.
impl Decode<'_, Postgres> for PgNumeric {
    fn decode(value: PgValue) -> crate::Result<Self> {
        if let PgData::Binary(bytes) = value.try_get()? {
            Self::from_bytes(bytes)
        } else {
            Err(Error::Decode(
                format!("`PgNumeric` can only be decoded from the binary protocol").into(),
            ))
        }
    }
}

/// ### Panics
///
/// * If `digits.len()` overflows `i16`
/// * If any element in `digits` is greater than or equal to 10000
impl Encode<Postgres> for PgNumeric {
    fn encode(&self, buf: &mut PgRawBuffer) {
        match *self {
            PgNumeric::Number {
                ref digits,
                sign,
                scale,
                weight,
            } => {
                let digits_len: i16 = digits
                    .len()
                    .try_into()
                    .expect("PgNumeric.digits.len() should not overflow i16");

                buf.put_i16::<BigEndian>(digits_len);
                buf.put_i16::<BigEndian>(weight);
                buf.put_i16::<BigEndian>(sign as i16);
                buf.put_i16::<BigEndian>(scale);
                for &digit in digits {
                    assert!(digit < 10000, "PgNumeric digits must be in base-10000");
                    buf.put_i16::<BigEndian>(digit);
                }
            }
            PgNumeric::NotANumber => {
                buf.put_i16::<BigEndian>(0);
                buf.put_i16::<BigEndian>(0);
                buf.put_u16::<BigEndian>(SIGN_NAN);
                buf.put_i16::<BigEndian>(0);
            }
        }
    }
    fn size_hint(&self) -> usize {
        // 4 i16's plus digits
        8 + if let PgNumeric::Number { digits, .. } = self {
            digits.len() * 2
        } else {
            0
        }
    }
}
