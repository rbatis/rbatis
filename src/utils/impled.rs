/// Placeholder body for functions handled by `#[py_sql]`, `#[html_sql]` or `#[sql]`.
///
/// Write [`impled!`](crate::impled) as the body of every generated function; the attribute macro
/// replaces it with the code that runs the statement:
///
/// ```rust,ignore
/// #[rbatis::py_sql("select * from biz_activity where id = #{id}")]
/// async fn select_by_id(rb: &dyn rbatis::Executor, id: &str) -> rbatis::Result<Vec<BizActivity>> {
///     impled!()
/// }
/// ```
#[macro_export]
macro_rules! impled {
    () => {
        Default::default()
    };
}
