use crate::error::Error;
use std::str::FromStr;

/// Refer to [SQLite documentation] for the meaning of the connection locking mode.
///
/// [SQLite documentation]: https://www.sqlite.org/pragma.html#pragma_locking_mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteLockingMode {
    Normal,
    Exclusive,
}

impl SqliteLockingMode {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SqliteLockingMode::Normal => "NORMAL",
            SqliteLockingMode::Exclusive => "EXCLUSIVE",
        }
    }
}

impl Default for SqliteLockingMode {
    fn default() -> Self {
        SqliteLockingMode::Normal
    }
}

impl FromStr for SqliteLockingMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match &*s.to_ascii_lowercase() {
            "normal" => SqliteLockingMode::Normal,
            "exclusive" => SqliteLockingMode::Exclusive,

            _ => {
                return Err(Error::Configuration(
                    format!("unknown value {:?} for `locking_mode`", s).into(),
                ));
            }
        })
    }
}
