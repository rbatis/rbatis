use crate::executor::Executor;
use crate::intercept::{Intercept, ResultType};
use crate::{Error, IPageRequest, PageRequest};
use async_trait::async_trait;
use dark_std::sync::SyncHashMap;
use rbdc::db::ExecResult;
use rbs::Value;
use std::sync::Arc;

/// make count sql remove `limit`
/// make select sql append limit ${page_no},${page_size}
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
    pub select_ids: Arc<SyncHashMap<i64, PageRequest>>,
    pub count_ids: Arc<SyncHashMap<i64, PageRequest>>,
}

impl PageIntercept {
    pub fn new() -> PageIntercept {
        Self {
            select_ids: Arc::new(SyncHashMap::new()),
            count_ids: Arc::new(SyncHashMap::new()),
        }
    }
}
#[async_trait]
impl Intercept for PageIntercept {
    async fn before(
        &self,
        _task_id: i64,
        executor: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if let ResultType::Exec(_) = result {
            return Ok(Some(true));
        }
        if self.count_ids.contains_key(&executor.id()) {
            self.count_ids.remove(&executor.id());
            if !(sql.contains("select ") && sql.contains(" from ")) {
                return Err(Error::from(
                    "InterceptPageExecutor sql must have select and from ",
                ));
            }
            let start = sql.find("select ").unwrap_or(0) + "select ".len();
            let end = sql.find(" from ").unwrap_or(0);
            let v = &sql[start..end];
            *sql = sql.replace(v, "count(1) as count").to_string();
            if let Some(idx) = sql.rfind(" limit ") {
                *sql = (&sql[..idx]).to_string();
            }
        }
        if self.select_ids.contains_key(&executor.id()) {
            let req = self.select_ids.remove(&executor.id());
            if !(sql.contains("select ") && sql.contains(" from ")) {
                return Err(Error::from(
                    "InterceptPageExecutor sql must have select and from ",
                ));
            }
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
                templete = " offset ${page_no} rows fetch next ${page_size} rows only ".to_string();
            }
            if !sql.contains("limit") {
                if let Some(req) = req {
                    templete = templete.replace("${page_no}", &req.offset().to_string());
                    templete = templete.replace("${page_size}", &req.page_size().to_string());
                    sql.push_str(&templete);
                }
            }
        }
        Ok(Some(true))
    }
}
