use crate::executor::Executor;
use crate::intercept::{Intercept, ResultType};
use crate::{Action, Error, IPageRequest, PageRequest};
use async_trait::async_trait;
use dark_std::sync::SyncHashMap;
use rbdc::db::ExecResult;
use rbs::Value;
use std::sync::Arc;

/// make count sql remove `limit`
/// make select sql append limit ${page_no},${page_size}
/// notice:
/// ```log
/// sql must be starts with 'select ' and ' from '
/// this PageIntercept only support sqlite,mysql,mssql,postgres...
///```
/// how to use?
/// ```rust
///
/// use rbatis::{crud, Error, PageRequest, RBatis};
/// use rbatis::intercept_page::PageIntercept;
/// #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///    pub id:Option<String>,
///    pub name:Option<String>,
/// }
/// rbatis::htmlsql_select_page!(select_page_data(name: &str) -> MockTable => r#"
/// <select id="select_page_data">
///  `select * from table where name = #{name} `
/// </select>"#);
///
///  async fn test_use_page_intercept(rb:&RBatis) -> Result<(),Error> {
///     let page = select_page_data(rb,&PageRequest::new(1,10),"a").await?;
///     println!("{:?}",page);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct PageIntercept {
    /// executors (connection or transaction ids) whose next query is a page select
    pub select_ids: Arc<SyncHashMap<i64, PageRequest>>,
    /// executors (connection or transaction ids) whose next query is a `count(1)`
    pub count_ids: Arc<SyncHashMap<i64, PageRequest>>,
}

impl Default for PageIntercept {
    fn default() -> Self {
        Self::new()
    }
}

impl PageIntercept {
    /// create the interceptor, [`RBatis::new`](crate::RBatis::new) does this for you
    pub fn new() -> PageIntercept {
        Self {
            select_ids: Arc::new(SyncHashMap::new()),
            count_ids: Arc::new(SyncHashMap::new()),
        }
    }

    //driver_type=['postgres','pg','mssql','mysql','sqlite'...],but sql default is use '?'(Driver will replace to pg='$1' or mssql='@p1' or sqlite/mysql '?' )
    /// number of `?` placeholders in the part of the SQL that has to be removed
    /// when the query is turned into a `count(1)` query
    pub fn count_param_count(&self, _driver_type: &str, sql: &str) -> usize {
        sql.matches('?').count()
    }
}
#[async_trait]
impl Intercept for PageIntercept {
    async fn before(
        &self,
        _task_id: i64,
        executor: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        if let ResultType::Exec(_) = result {
            return Ok(Action::Next);
        }
        if self.count_ids.contains_key(&executor.id()) {
            self.count_ids.remove(&executor.id());
            if sql.trim_start().starts_with("select ") && sql.contains(" from ") {
                // replace just the column list of the first 'select ... from' with
                // 'count(1) as count'. sql.replace() would also rewrite the same text
                // when it appears again inside a string literal or a sub query
                let start = sql.find("select ").unwrap_or(0) + "select ".len();
                let end = sql.find(" from ").unwrap_or(0);
                if end > start {
                    *sql = format!("{}count(1) as count{}", &sql[..start], &sql[end..]);
                }
                // a count(1) query needs no limit and no order by. The arguments that
                // belonged to the removed tail are dropped as well, otherwise their
                // '?' placeholders are gone but the values stay in args
                let driver_type = executor.driver_type().unwrap_or_default();
                for tail_key in [" limit ", " order by "] {
                    if let Some(idx) = sql.rfind(tail_key) {
                        let removed = &sql[idx..];
                        let param_count = self.count_param_count(driver_type, removed);
                        // remove the bound arguments of the removed sql (args.pop())
                        for _ in 0..param_count {
                            args.pop();
                        }
                        *sql = sql[..idx].to_string();
                    }
                }
            }
        }
        if self.select_ids.contains_key(&executor.id()) {
            let req = self.select_ids.remove(&executor.id());
            if sql.trim_start().starts_with("select ") && sql.contains(" from ") {
                let driver_type = executor.driver_type().unwrap_or_default();
                let mut templete = " limit ${page_no},${page_size} ".to_string();
                if driver_type == "pg" || driver_type == "postgres" {
                    //postgres use `limit x offset x`
                    templete = " limit ${page_size} offset ${page_no}".to_string();
                } else if driver_type == "mssql" {
                    //mssql must have `order by`, if you not add on sql.we will add this
                    if !sql.contains(" order by ") {
                        sql.push_str(" order by id desc ");
                    }
                    templete =
                        " offset ${page_no} rows fetch next ${page_size} rows only ".to_string();
                }
                if !sql.contains(" limit ") {
                    if let Some(req) = req {
                        templete = templete.replace("${page_no}", &req.offset().to_string());
                        templete = templete.replace("${page_size}", &req.page_size().to_string());
                        sql.push_str(&templete);
                    }
                }
            }
        }
        Ok(Action::Next)
    }
}
