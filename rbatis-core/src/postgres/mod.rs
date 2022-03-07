use serde::{Deserialize, Serialize};

mod sqlx_value;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PgInterval {
    pub months: i32,
    pub days: i32,
    pub microseconds: i64,
}

impl From<sqlx_core::postgres::types::PgInterval> for PgInterval {
    fn from(arg: sqlx_core::postgres::types::PgInterval) -> Self {
        Self {
            months: arg.months,
            days: arg.days,
            microseconds: arg.microseconds,
        }
    }
}
