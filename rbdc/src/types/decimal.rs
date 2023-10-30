use crate::Error;
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use rbs::Value;
use serde::Deserializer;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Decimal")]
pub struct Decimal(pub BigDecimal);

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)?.into_string() {
            None => Err(D::Error::custom("warn type decode Decimal")),
            Some(v) => Ok(Decimal::from_str(&v).map_err(|e| D::Error::custom(e.to_string())))?,
        }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decimal({})", self.0)
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decimal({})", self.0)
    }
}

impl From<Decimal> for Value {
    fn from(arg: Decimal) -> Self {
        Value::Ext("Decimal", Box::new(Value::String(arg.0.to_string())))
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal(BigDecimal::from_str(&s)?))
    }
}

impl From<ParseBigDecimalError> for Error {
    fn from(value: ParseBigDecimalError) -> Self {
        Error::E(value.to_string())
    }
}

impl Deref for Decimal {
    type Target = BigDecimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Decimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Decimal(BigDecimal::from(0))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Decimal(self.0.add(rhs.0))
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0.sub(rhs.0))
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(self.0.mul(rhs.0))
    }
}

impl Div for Decimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Decimal(self.0.div(rhs.0))
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Rem for Decimal {
    type Output = Decimal;

    fn rem(self, other: Decimal) -> Decimal {
        Decimal(self.0.rem(other.0))
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Self::Output {
        Decimal(self.0.neg())
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}

impl MulAssign for Decimal {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0)
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::Decimal;
    use std::str::FromStr;
    use rbs::{from_value, to_value};

    #[test]
    fn test_big_decimal() {
        let v1 = Decimal::from_str("1").unwrap();
        let v2 = Decimal::from_str("1.1").unwrap();
        let v = v1 + v2;
        assert_eq!(v, Decimal::from_str("2.1").unwrap());
    }

    #[test]
    fn test_ser() {
        let v1 = Decimal::from_str("1").unwrap();
        let rv:Decimal = from_value(to_value!(v1)).unwrap();
        assert_eq!(rv,Decimal::from_str("1").unwrap());
    }

    #[test]
    fn test_ser2() {
        let v1 = Decimal::from_str("1").unwrap();
        let rv:Decimal = serde_json::from_value(serde_json::to_value(v1).unwrap()).unwrap();
        assert_eq!(rv,Decimal::from_str("1").unwrap());
    }
}
