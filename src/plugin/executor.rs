use crate::executor::{Executor, RBatisRef};
use crate::{Error, IPageRequest, PageRequest, RBatis};
use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbs::Value;

/// PageCountExecutor
/// `select * from` to `select count(1) as count from `
/// `limit xx,xx` to ``
/// `order by xx desc` to ``
pub struct PageCountExecutor {
    pub inner: &'static dyn Executor,
}

impl PageCountExecutor {
    pub fn new(inner: &dyn Executor) -> PageCountExecutor {
        Self {
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

/// PageCountExecutor
/// `select * from table ` to `select * from table limit 1,10`
pub struct PageSelectExecutor {
    pub inner: &'static dyn Executor,
    pub page_req: PageRequest,
}

impl PageSelectExecutor {
    pub fn new(inner: &dyn Executor, req: &dyn IPageRequest) -> PageSelectExecutor {
        Self {
            inner: unsafe { std::mem::transmute::<&dyn Executor, &'static dyn Executor>(inner) },
            page_req: PageRequest::new(req.page_no(), req.page_size()),
        }
    }

    pub fn executor_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Executor for PageSelectExecutor {
    fn id(&self) -> i64 {
        self.inner.id()
    }

    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        self.inner.exec(&sql, args)
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            if !(sql.contains("select ") && sql.contains(" from ")) {
                return Err(Error::from(
                    "InterceptPageExecutor sql must have select and from ",
                ));
            }
            let driver_type = self.rb_ref().driver_type().unwrap_or_default();
            let mut templete = " limit ${page_no},${page_size} ".to_string();
            if driver_type == "pg" || driver_type == "postgres" {
                templete = " limit ${page_size} offset ${page_no}".to_string();
            } else if driver_type == "mssql" {
                if !sql.contains(" order by ") {
                    sql.push_str(" order by id desc ");
                }
                templete = " offset ${page_no} rows fetch next ${page_size} rows only ".to_string();
            }
            if !sql.contains("limit") {
                templete = templete.replace("${page_no}", &self.page_req.offset().to_string());
                templete = templete.replace("${page_size}", &self.page_req.page_size().to_string());
                sql.push_str(&templete);
            }
            let result = self.inner.query(&sql, args).await;
            return result;
        })
    }
}

impl RBatisRef for PageSelectExecutor {
    fn rb_ref(&self) -> &RBatis {
        self.inner.rb_ref()
    }
}
