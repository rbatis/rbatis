use crate::executor::Executor;
use crate::intercept::{Intercept, ResultType};
use crate::{Error, IPageRequest, PageRequest};
use async_trait::async_trait;
use rbdc::db::ExecResult;
use rbs::Value;

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
    pub req: PageRequest,
}

impl PageIntercept {
    pub fn new(req: PageRequest) -> PageIntercept {
        Self { req }
    }

    //driver_type=['postgres','pg','mssql','mysql','sqlite'...],but sql default is use '?'(Driver will replace to pg='$1' or mssql='@p1' or sqlite/mysql '?' )
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
        _args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if let ResultType::Exec(_) = result {
            return Ok(Some(true));
        }
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
                templete = " offset ${page_no} rows fetch next ${page_size} rows only ".to_string();
            }
            if !sql.contains(" limit ") {
                templete = templete.replace("${page_no}", &self.req.offset().to_string());
                templete = templete.replace("${page_size}", &self.req.page_size().to_string());
                sql.push_str(&templete);
            }
        }
        Ok(Some(true))
    }
}

#[derive(Debug)]
pub struct PageInterceptCount {
    pub req: PageRequest,
}

impl PageInterceptCount {
    pub fn new(req: PageRequest) -> PageInterceptCount {
        Self { req }
    }

    //driver_type=['postgres','pg','mssql','mysql','sqlite'...],but sql default is use '?'(Driver will replace to pg='$1' or mssql='@p1' or sqlite/mysql '?' )
    pub fn count_param_count(&self, _driver_type: &str, sql: &str) -> usize {
        sql.matches('?').count()
    }
}
#[async_trait]
impl Intercept for PageInterceptCount {
    async fn before(
        &self,
        _task_id: i64,
        executor: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if let ResultType::Exec(_) = result {
            return Ok(Some(true));
        }
        if sql.trim_start().starts_with("select ") && sql.contains(" from ") {
            let start = sql.find("select ").unwrap_or(0) + "select ".len();
            let end = sql.find(" from ").unwrap_or(0);
            let v = &sql[start..end];
            *sql = sql.replace(v, "count(1) as count").to_string();
            if let Some(idx) = sql.rfind(" limit ") {
                *sql = (&sql[..idx]).to_string();
            }
            if let Some(idx) = sql.rfind(" order by ") {
                //remove args(args.pop())
                let order_by_clause = &sql[idx..];
                let driver_type = executor.driver_type().unwrap_or_default();
                let param_count = self.count_param_count(driver_type, &order_by_clause);
                // 移除对应的参数
                for _ in 0..param_count {
                    args.pop();
                }
                *sql = (&sql[..idx]).to_string();
            }
        }
        Ok(Some(true))
    }
}
