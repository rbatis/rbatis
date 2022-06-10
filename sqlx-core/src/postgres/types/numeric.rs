use bytes::Buf;

use crate::error::BoxDynError;
use crate::postgres::PgArgumentBuffer;

/// Represents a `NUMERIC` value in the **Postgres** wire protocol.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PgNumeric {
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
pub(crate) enum PgNumericSign {
    Positive = SIGN_POS,
    Negative = SIGN_NEG,
}

impl PgNumericSign {
    fn try_from_u16(val: u16) -> Result<Self, BoxDynError> {
        match val {
            SIGN_POS => Ok(PgNumericSign::Positive),
            SIGN_NEG => Ok(PgNumericSign::Negative),

            SIGN_NAN => unreachable!("sign value for NaN passed to PgNumericSign"),

            _ => Err(format!("invalid value for PgNumericSign: {:#04X}", val).into()),
        }
    }
}

impl PgNumeric {
    pub(crate) fn decode(mut buf: &[u8]) -> Result<Self, BoxDynError> {
        // https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L874
        let num_digits = buf.get_u16();
        let weight = buf.get_i16();
        let sign = buf.get_u16();
        let scale = buf.get_i16();

        if sign == SIGN_NAN {
            Ok(PgNumeric::NotANumber)
        } else {
            let digits: Vec<_> = (0..num_digits).map(|_| buf.get_i16()).collect::<_>();

            Ok(PgNumeric::Number {
                sign: PgNumericSign::try_from_u16(sign)?,
                scale,
                weight,
                digits,
            })
        }
    }

    /// ### Panics
    ///
    /// * If `digits.len()` overflows `i16`
    /// * If any element in `digits` is greater than or equal to 10000
    pub(crate) fn encode(&self, buf: &mut PgArgumentBuffer) {
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

                buf.extend(&digits_len.to_be_bytes());
                buf.extend(&weight.to_be_bytes());
                buf.extend(&(sign as i16).to_be_bytes());
                buf.extend(&scale.to_be_bytes());

                for digit in digits {
                    debug_assert!(*digit < 10000, "PgNumeric digits must be in base-10000");

                    buf.extend(&digit.to_be_bytes());
                }
            }

            PgNumeric::NotANumber => {
                buf.extend(&0_i16.to_be_bytes());
                buf.extend(&0_i16.to_be_bytes());
                buf.extend(&SIGN_NAN.to_be_bytes());
                buf.extend(&0_i16.to_be_bytes());
            }
        }
    }
}
