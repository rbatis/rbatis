use crate::Error;

pub mod bytes;
///this mod support Some common types, the actual type may need to use the type provided by the driver
///
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod json;
pub mod time;
pub mod timestamp;
pub mod uuid;

pub trait RBDCString {
    fn ends_name() -> &'static str
    where
        Self: Sized;

    fn trim_ends_match(v: &mut String)
    where
        Self: Sized,
    {
        if v.ends_with(Self::ends_name()) {
            for _ in 0..Self::ends_name().len() {
                v.pop();
            }
        }
    }

    fn is(arg: &str) -> &str
    where
        Self: Sized,
    {
        if arg.ends_with(Self::ends_name()) {
            Self::ends_name()
        } else {
            ""
        }
    }

    fn encode_str(&self, arg: &str, result: &mut String)
    where
        Self: Sized,
    {
        if !arg.ends_with(Self::ends_name()) {
            result.push_str(Self::ends_name());
        }
    }

    fn decode_str(arg: &str) -> Result<Self, Error>
    where
        Self: Sized;
}
