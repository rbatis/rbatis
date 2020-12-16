use crate::convert::ResultCodec;
use sqlx_core::error::BoxDynError;

/// convert sqlx-Result to rbatis-core Result
impl<T> ResultCodec<T> for Result<T, BoxDynError> {
    fn into_result(self) -> crate::Result<T> {
        match self {
            Ok(t) => {
                return Ok(t);
            }
            Err(e) => {
                return Err(crate::Error::from(e.to_string()));
            }
        }
    }
}