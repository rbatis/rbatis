use crate::rbatis::Rbatis;
use crate::core::db::{DBExecResult, DBPool, DBPoolConn, DBPoolOptions, DBQuery, DBTx, DriverType};
use crate::core::Error;
use serde::de::DeserializeOwned;
use crate::utils::string_util;
use serde::Serialize;
use crate::plugin::page::{IPageRequest, Page, IPage};

impl Rbatis{
    /// fetch result(row sql)
    ///
    /// for example:
    ///     let v: serde_json::Value = rb.fetch(context_id, "select count(1) from biz_activity;").await?;
    ///
    pub async fn fetch<T>(&self, context_id: &str, sql: &str) -> Result<T, Error>
        where
            T: DeserializeOwned,
    {
        //sql intercept
        let mut sql = sql.to_string();
        for item in &self.sql_intercepts {
            item.do_intercept(self, context_id, &mut sql, &mut vec![], false)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin
                .info(context_id, &format!("Query ==> {}", sql.as_str()));
        }
        let result: Result<(T, usize), Error>;
        if self.tx_manager.is_tx_id(context_id) {
            let g = self.tx_manager.tx_context.read().await;
            let conn = g.get(context_id).ok_or_else(|| {
                Error::from(format!(
                    "[rbatis] transaction:{} not exist！",
                    context_id
                ))
            })?;
            result = conn.lock().await.tx.fetch(sql.as_str()).await;
            drop(g);
        } else {
            let mut conn = self.get_pool()?.acquire().await?;
            result = conn.fetch(sql.as_str()).await;
        }
        if self.log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.log_plugin
                        .info(context_id, &format!("ReturnRows <== {}", result.1));
                }
                Err(e) => {
                    self.log_plugin
                        .error(context_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return Ok(result?.0);
    }

    /// exec sql(row sql)
    /// for example:
    ///     rb.exec("", "CREATE TABLE biz_uuid( id uuid, name VARCHAR, PRIMARY KEY(id));").await;
    ///
    pub async fn exec(&self, context_id: &str, sql: &str) -> Result<DBExecResult, Error> {
        //sql intercept
        let mut sql = sql.to_string();
        for item in &self.sql_intercepts {
            item.do_intercept(self, context_id, &mut sql, &mut vec![], false)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin
                .info(context_id, &format!("Exec  ==> {}", &sql));
        }
        let result;
        if self.tx_manager.is_tx_id(context_id) {
            let g = self.tx_manager.tx_context.read().await;
            let conn = g.get(context_id).ok_or_else(|| {
                Error::from(format!(
                    "[rbatis] transaction:{} not exist！",
                    context_id
                ))
            })?;
            result = conn.lock().await.tx.execute(&sql).await;
            drop(g);
        } else {
            let mut conn = self.get_pool()?.acquire().await?;
            result = conn.execute(&sql).await;
        }
        if self.log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.log_plugin.info(
                        context_id,
                        &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.log_plugin
                        .error(context_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    /// bind arg into DBQuery
    pub fn bind_arg<'arg>(
        &self,
        sql: &'arg str,
        arg: &Vec<serde_json::Value>,
    ) -> Result<DBQuery<'arg>, Error> {
        let mut q: DBQuery = self.get_pool()?.make_query(sql)?;
        for x in arg {
            q.bind_value(x);
        }
        return Ok(q);
    }

    /// fetch result(prepare sql)
    ///
    /// for example:
    ///     let v = RB.fetch_prepare::<Value>("", "select count(1) from biz_activity where delete_flag = ?;", &vec![json!(1)]).await;
    ///
    pub async fn fetch_prepare<T>(
        &self,
        context_id: &str,
        sql: &str,
        args: &Vec<serde_json::Value>,
    ) -> Result<T, Error>
        where
            T: DeserializeOwned,
    {
        //sql intercept
        let mut sql = sql.to_string();
        let mut args = args.clone();
        for item in &self.sql_intercepts {
            item.do_intercept(self, context_id, &mut sql, &mut args, true)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin.info(
                context_id,
                &format!(
                    "Query ==> {}\n{}[rbatis] [{}] Args  ==> {}",
                    &sql,
                    string_util::LOG_SPACE,
                    context_id,
                    serde_json::Value::Array(args.clone()).to_string()
                ),
            );
        }
        let result: Result<(T, usize), Error>;
        if self.tx_manager.is_tx_id(context_id) {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let g = self.tx_manager.tx_context.read().await;
            let conn = g.get(context_id).ok_or_else(|| {
                Error::from(format!(
                    "[rbatis] transaction:{} not exist！",
                    context_id
                ))
            })?;
            result = conn.lock().await.tx.fetch_parperd(q).await;
            drop(g);
        } else {
            let mut conn = self.get_pool()?.acquire().await?;
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            result = conn.fetch_parperd(q).await;
        }
        if self.log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.log_plugin
                        .info(context_id, &format!("ReturnRows <== {}", result.1));
                }
                Err(e) => {
                    self.log_plugin
                        .error(context_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return Ok(result?.0);
    }

    /// exec sql(prepare sql)
    ///
    /// for example:
    ///      let v = RB.exec_prepare::<Value>("", "select count(1) from biz_activity where delete_flag = ?;", &vec![json!(1)]).await;
    ///
    #[deprecated]
    pub async fn exec_prepare(
        &self,
        context_id: &str,
        sql: &str,
        args: &Vec<serde_json::Value>,
    ) -> Result<DBExecResult, Error> {
        //sql intercept
        let mut sql = sql.to_string();
        let mut args = args.clone();
        for item in &self.sql_intercepts {
            item.do_intercept(self, context_id, &mut sql, &mut args, true)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin.info(
                context_id,
                &format!(
                    "Exec  ==> {}\n{}[rbatis] [{}] Args  ==> {}",
                    &sql,
                    string_util::LOG_SPACE,
                    context_id,
                    serde_json::Value::Array(args.clone()).to_string()
                ),
            );
        }
        let result;
        if self.tx_manager.is_tx_id(context_id) {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let g = self.tx_manager.tx_context.read().await;
            let conn = g.get(context_id).ok_or_else(|| {
                Error::from(format!(
                    "[rbatis] transaction:{} not exist！",
                    context_id
                ))
            })?;
            result = conn.lock().await.tx.exec_prepare(q).await;
            drop(g);
        } else {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let mut conn = self.get_pool()?.acquire().await?;
            result = conn.exec_prepare(q).await;
        }
        if self.log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.log_plugin.info(
                        context_id,
                        &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.log_plugin
                        .error(context_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

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
            .runtime_py
            .eval(&self.driver_type()?, py_sql, &mut arg, &self.runtime_expr)
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
    pub async fn py_fetch<T, Arg>(
        &self,
        context_id: &str,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<T, Error>
        where
            T: DeserializeOwned,
            Arg: Serialize + Send + Sync,
    {
        let (sql, args) = self.py_to_sql(py_sql, arg)?;
        return self.fetch_prepare(context_id, sql.as_str(), &args).await;
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
    pub async fn py_exec<Arg>(
        &self,
        context_id: &str,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<DBExecResult, Error>
        where
            Arg: Serialize + Send + Sync,
    {
        let (sql, args) = self.py_to_sql(py_sql, arg)?;
        return self.exec_prepare(context_id, sql.as_str(), &args).await;
    }

    /// fetch page result(prepare sql)
    pub async fn fetch_page<T>(
        &self,
        context_id: &str,
        sql: &str,
        args: &Vec<serde_json::Value>,
        page_request: &dyn IPageRequest,
    ) -> Result<Page<T>, Error>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
    {
        let mut page_result = Page::new(page_request.get_page_no(), page_request.get_page_size());
        page_result.search_count = page_request.is_search_count();
        let (count_sql, sql) = self.page_plugin.make_page_sql(
            &self.driver_type()?,
            context_id,
            &sql,
            args,
            page_request,
        )?;
        if page_request.is_search_count() {
            //make count sql
            let total: Option<u64> = self
                .fetch_prepare(context_id, count_sql.as_str(), args)
                .await?;
            page_result.set_total(total.unwrap_or(0));
            page_result.pages = page_result.get_pages();
            if page_result.get_total() == 0 {
                return Ok(page_result);
            }
        }
        let data: Option<Vec<T>> = self.fetch_prepare(context_id, sql.as_str(), args).await?;
        page_result.set_records(data.unwrap_or(vec![]));
        page_result.pages = page_result.get_pages();
        return Ok(page_result);
    }

    /// fetch result(prepare sql)
    pub async fn py_fetch_page<T, Arg>(
        &self,
        context_id: &str,
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
            .fetch_page::<T>(context_id, sql.as_str(), &args, page_request)
            .await;
    }
}