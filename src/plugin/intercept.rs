/// sql intercept
pub trait SqlIntercept: Send + Sync {
    ///intercept name
    fn name(&self) -> &str;
    /// do intercept sql/args
    fn do_intercept(&self, sql: &mut String, args: &mut Vec<serde_json::Value>) -> Result<(), rbatis_core::Error>;
}