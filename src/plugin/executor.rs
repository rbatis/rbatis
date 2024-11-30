use crate::executor::{Executor, RBatisRef};
use crate::{Error, RBatis};
use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbs::Value;

/// PageCountExecutor
/// `select * from` to `select count(1) as count from `
/// `limit xx,xx` to ``
/// `order by xx desc` to ``
pub struct PageCountExecutor {
    pub name: String,
    pub inner: &'static dyn Executor,
}

impl PageCountExecutor {
    pub fn new(inner: &dyn Executor) -> PageCountExecutor {
        Self {
            name: "InterceptPageExecutor".to_string(),
            inner: unsafe { std::mem::transmute::<&dyn Executor, &'static dyn Executor>(inner) },
        }
    }

    pub fn executor_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Executor for PageCountExecutor {
    fn id(&self) -> i64 {
        self.inner.id()
    }

    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        self.inner.exec(&sql, args)
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            if !(sql.contains("select ") && sql.contains(" from ")) {
                return Err(Error::from(
                    "InterceptPageExecutor sql must have select and from ",
                ));
            }
            let start = sql.find("select ").unwrap_or(0) + "select ".len();
            let end = sql.find(" from ").unwrap_or(0);
            let v = &sql[start..end];
            let mut sql = sql.replace(v, "count(1) as count").to_string();
            if let Some(idx) = sql.rfind(" limit ") {
                sql = (&sql[..idx]).to_string();
            }
            if let Some(idx) = sql.rfind(" order by ") {
                sql = (&sql[..idx]).to_string();
            }
            let result = self.inner.query(&sql, args).await;
            return result;
        })
    }
}

impl RBatisRef for PageCountExecutor {
    fn rb_ref(&self) -> &RBatis {
        self.inner.rb_ref()
    }
}
