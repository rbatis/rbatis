use crate::rbatis::Rbatis;

/// sql intercept
pub trait SqlIntercept: Send + Sync {
    ///the intercept name
    fn name(&self) -> &str;
    /// do intercept sql/args
    /// is_prepared_sql: if is run in prepared_sql=ture
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<serde_json::Value>, is_prepared_sql: bool) -> Result<(), rbatis_core::Error>;
}