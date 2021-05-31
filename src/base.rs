use crate::rbatis::Rbatis;
use crate::core::db::{DBExecResult, DBPool, DBPoolConn, DBPoolOptions, DBQuery, DBTx, DriverType};
use crate::core::Error;
use serde::de::DeserializeOwned;
use crate::utils::string_util;
use serde::Serialize;
use crate::plugin::page::{IPageRequest, Page, IPage};
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor, Executor};
use async_trait::async_trait;

#[async_trait]
pub trait Base: Executor {
    /// py str into py ast,run get sql,arg result
    fn py_to_sql<Arg>(
        &self,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<(String, Vec<serde_json::Value>), Error>
        where
            Arg: Serialize + Send + Sync,
    {
        let mut arg = json!(arg);
        match self
            .get_rbatis()
            .runtime_py
            .eval(&self.driver_type()?, py_sql, &mut arg, &self.get_rbatis().runtime_expr)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// fetch query result(prepare sql)
    ///for example:
    ///
    ///         let py = r#"
    ///     select * from biz_activity
    ///    where delete_flag = #{delete_flag}
    ///     if name != null:
    ///       and name like #{name+'%'}
    ///     if ids != null:
    ///       and id in (
    ///       trim ',':
    ///          for item in ids:
    ///            #{item},
    ///       )"#;
    ///         let data: serde_json::Value = rb.py_fetch("", py, &json!({   "delete_flag": 1 })).await.unwrap();
    ///
    async fn py_fetch<T, Arg>(
        &mut self,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<T, Error>
        where
            T: DeserializeOwned,
            Arg: Serialize + Send + Sync,
    {
        let (sql, args) = self.py_to_sql(py_sql, arg)?;
        return self.fetch(sql.as_str(), &args).await;
    }

    /// exec sql(prepare sql)
    ///for example:
    ///
    ///         let py = r#"
    ///     select * from biz_activity
    ///    where delete_flag = #{delete_flag}
    ///     if name != null:
    ///       and name like #{name+'%'}
    ///     if ids != null:
    ///       and id in (
    ///       trim ',':
    ///          for item in ids:
    ///            #{item},
    ///       )"#;
    ///         let data: u64 = rb.py_exec("", py, &json!({   "delete_flag": 1 })).await.unwrap();
    ///
    async fn py_exec<Arg>(
        &mut self,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<DBExecResult, Error>
        where
            Arg: Serialize + Send + Sync,
    {
        let (sql, args) = self.py_to_sql(py_sql, arg)?;
        return self.execute(sql.as_str(), &args).await;
    }

    /// fetch page result(prepare sql)
    async fn fetch_page<T>(
        &mut self,
        sql: &str,
        args: &Vec<serde_json::Value>,
        page_request: &dyn IPageRequest,
    ) -> Result<Page<T>, Error>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
    {
        let mut page_result = Page::new(page_request.get_page_no(), page_request.get_page_size());
        page_result.search_count = page_request.is_search_count();
        let (count_sql, sql) = self.get_rbatis().page_plugin.make_page_sql(
            &self.driver_type()?,
            "",
            &sql,
            args,
            page_request,
        )?;
        if page_request.is_search_count() {
            //make count sql
            let total: Option<u64> = self
                .fetch(&count_sql, args)
                .await?;
            page_result.set_total(total.unwrap_or(0));
            page_result.pages = page_result.get_pages();
            if page_result.get_total() == 0 {
                return Ok(page_result);
            }
        }
        let data: Option<Vec<T>> = self.fetch(sql.as_str(), args).await?;
        page_result.set_records(data.unwrap_or(vec![]));
        page_result.pages = page_result.get_pages();
        return Ok(page_result);
    }

    /// fetch result(prepare sql)
    async fn py_fetch_page<T, Arg>(
        &mut self,
        py_sql: &str,
        arg: &Arg,
        page_request: &dyn IPageRequest,
    ) -> Result<Page<T>, Error>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
            Arg: Serialize + Send + Sync,
    {
        let (sql, args) = self.py_to_sql(py_sql, arg)?;
        return self
            .fetch_page::<T>(sql.as_str(), &args, page_request)
            .await;
    }
}

impl<'a> Base for RBatisConnExecutor<'a> {}

impl<'a> Base for RBatisTxExecutor<'a> {}