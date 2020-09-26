use std::cell::Cell;
use std::collections::HashMap;

use dashmap::DashMap;
use log::{error, info, LevelFilter, warn};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use rbatis_core::connection::Connection;
use rbatis_core::cursor::Cursor;
use rbatis_core::db::{DBPool, DBPoolConn, DBQuery, DBTx, DriverType, PoolOptions};
use rbatis_core::Error;
use rbatis_core::executor::Executor;
use rbatis_core::pool::{Pool, PoolConnection};
use rbatis_core::query::{query, Query};
use rbatis_core::query_as::query_as;
use rbatis_core::transaction::Transaction;

use crate::ast::ast::RbatisAST;
use crate::ast::lang::py::Py;
use crate::ast::lang::xml::Xml;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::node::do_child_nodes;
use crate::ast::node::node_type::NodeType;
use crate::ast::node::select_node::SelectNode;
use crate::ast::node::update_node::UpdateNode;
use crate::engine::runtime::RbatisEngine;
use crate::plugin::intercept::SqlIntercept;
use crate::plugin::logic_delete::{LogicDelete, RbatisLogicDeletePlugin};
use crate::plugin::page::{IPage, IPageRequest, Page, PagePlugin, RbatisPagePlugin};
use crate::sql::PageLimit;
use crate::utils::error_util::ToResult;
use crate::wrapper::Wrapper;

/// rbatis engine
pub struct Rbatis {
    pub pool: OnceCell<DBPool>,
    pub engine: RbatisEngine,
    // map<mapper_name,map<method_name,NodeType>>
    pub mapper_node_map: HashMap<String, HashMap<String, NodeType>>,
    //context of tx
    pub tx_context: DashMap<String, DBTx>,
    // page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    // sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    // logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
}

impl<'r> Default for Rbatis {
    fn default() -> Rbatis {
        Rbatis::new()
    }
}

impl Rbatis {
    pub fn new() -> Self {
        return Self {
            pool: OnceCell::new(),
            mapper_node_map: HashMap::new(),
            engine: RbatisEngine::new(),
            tx_context: DashMap::new(),
            page_plugin: Box::new(RbatisPagePlugin {}),
            sql_intercepts: vec![],
            logic_plugin: None,
        };
    }

    /// try return an new wrapper,if not call the link() method,it will be panic!
    pub fn new_wrapper(&self) -> Wrapper {
        let driver = self.driver_type();
        if driver.as_ref().unwrap().eq(&DriverType::None) {
            panic!("[rbatis] .new_wrapper() method must be call .link(url) to init first!");
        }
        Wrapper::new(&driver.unwrap_or_else(|_| {
            panic!("[rbatis] .new_wrapper() method must be call .link(url) to init first!");
        }))
    }

    pub fn check(&self) {
        println!("self.pool: {:?}", self.pool);
        println!("self.mapper_node_map: {:?}", self.mapper_node_map);
    }

    /// link pool
    pub async fn link(&self, url: &str) -> Result<(), rbatis_core::Error> {
        if url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let pool = DBPool::new(url).await?;
        self.pool.get_or_init(|| {
            pool
        });
        return Ok(());
    }

    /// link pool by options
    /// for example:
    pub async fn link_opt(&self, url: &str, opt: &PoolOptions) -> Result<(), rbatis_core::Error> {
        if url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let pool = DBPool::new_opt(url, opt).await?;
        self.pool.get_or_init(|| {
            pool
        });
        return Ok(());
    }

    /// load xml data into rbatis
    pub fn load_xml(&mut self, mapper_name: &str, data: &str) -> Result<(), rbatis_core::Error> {
        let xml = Xml::parse(data);
        self.mapper_node_map.insert(mapper_name.to_string(), xml);
        return Ok(());
    }

    /// get conn pool
    pub fn get_pool(&self) -> Result<&DBPool, rbatis_core::Error> {
        let p = self.pool.get();
        if p.is_none() {
            return Err(rbatis_core::Error::from("[rbatis] rbatis pool not inited!"));
        }
        return Ok(p.unwrap());
    }

    /// get driver type
    pub fn driver_type(&self) -> Result<DriverType, rbatis_core::Error> {
        let pool = self.get_pool()?;
        Ok(pool.driver_type)
    }

    /// begin tx,for new conn
    pub async fn begin(&self, new_tx_id: &str) -> Result<u64, rbatis_core::Error> {
        if new_tx_id.is_empty() {
            return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn = self.get_pool()?.begin().await?;
        //send tx to context
        self.tx_context.insert(new_tx_id.to_string(), conn);
        info!("[rbatis] [{}] Begin", new_tx_id);
        return Ok(1);
    }

    /// begin tx,with an exist conn
    pub async fn begin_with_conn(&self, new_tx_id: &str, db_conn: DBPoolConn) -> Result<u64, rbatis_core::Error> {
        if new_tx_id.is_empty() {
            return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn = db_conn.begin().await?;
        //send tx to context
        self.tx_context.insert(new_tx_id.to_string(), conn);
        info!("[rbatis] [{}] Begin", new_tx_id);
        return Ok(1);
    }

    /// commit tx,and return conn
    pub async fn commit(&self, tx_id: &str) -> Result<DBPoolConn, rbatis_core::Error> {
        let tx = self.tx_context.remove(tx_id);
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (key, mut tx) = tx.unwrap();
        let result = tx.commit().await?;
        info!("[rbatis] [{}] Commit", tx_id);
        return Ok(result);
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, tx_id: &str) -> Result<DBPoolConn, rbatis_core::Error> {
        let tx_op = self.tx_context.remove(tx_id);
        if tx_op.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let (key, mut tx) = tx_op.unwrap();
        let result = tx.rollback().await?;
        info!("[rbatis] [{}] Rollback", tx_id);
        return Ok(result);
    }


    /// fetch result(row sql)
    pub async fn fetch<T>(&self, tx_id: &str, sql: &str) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {

        //sql intercept
        let mut sql = sql.to_string();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut vec![], false);
        }

        info!("[rbatis] [{}] Query ==> {}", tx_id, sql.as_str());
        let data;
        let fetch_num;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let mut c = conn.fetch(sql.as_str())?;
            let json = c.fetch_json().await?;
            fetch_num = json.len();
            data = rbatis_core::decode::json_decode::<T>(json)?;
        } else {
            let conn = self.tx_context.get_mut(tx_id);
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            let c = conn.fetch(sql.as_str());
            if c.is_err() {
                let e = c.err().unwrap();
                return Err(e);
            }
            let mut c = c.unwrap();
            let json = c.fetch_json().await?;
            fetch_num = json.len();
            data = rbatis_core::decode::json_decode::<T>(json)?;
        }
        info!("[rbatis] [{}] ReturnRows <== {}", tx_id, fetch_num);
        return Ok(data);
    }

    /// exec sql(row sql)
    pub async fn exec(&self, tx_id: &str, sql: &str) -> Result<u64, rbatis_core::Error> {

        //sql intercept
        let mut sql = sql.to_string();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut vec![], false);
        }

        info!("[rbatis] [{}] Exec ==> :{}", tx_id, &sql);
        let data;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            data = conn.execute(&sql).await?;
        } else {
            let conn = self.tx_context.get_mut(tx_id);
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            data = conn.execute(&sql).await?;
        }
        info!("[rbatis] [{}] RowsAffected <== {}", tx_id, &data);
        return Ok(data);
    }

    fn bind_arg<'a>(&self, sql: &'a str, arg: &Vec<serde_json::Value>) -> Result<DBQuery<'a>, rbatis_core::Error> {
        let mut q: DBQuery = self.get_pool()?.make_query(sql)?;
        for x in arg {
            q.bind_value(x);
        }
        return Ok(q);
    }

    /// fetch result(prepare sql)
    pub async fn fetch_prepare<T>(&self, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {

        //sql intercept
        let mut sql = sql.to_string();
        let mut args = args.clone();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, true);
        }

        info!("[rbatis] [{}] Query ==> {}", tx_id, &sql);
        info!("[rbatis] [{}] Args  ==> {}", tx_id, serde_json::to_string(&args).unwrap_or("".to_string()));
        let result;
        let return_num;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let mut c = conn.fetch_parperd(q)?;
            let json_array = c.fetch_json().await?;
            return_num = json_array.len();
            result = rbatis_core::decode::json_decode::<T>(json_array)?;
        } else {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let conn = self.tx_context.get_mut(tx_id);
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            let mut c = conn.fetch_parperd(q)?;
            let json = c.fetch_json().await?;
            return_num = json.len();
            result = rbatis_core::decode::json_decode::<T>(json)?;
        }
        info!("[rbatis] [{}] ReturnRows <== {}", tx_id, return_num);
        return Ok(result);
    }

    /// exec sql(prepare sql)
    pub async fn exec_prepare(&self, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<u64, rbatis_core::Error> {

        //sql intercept
        let mut sql = sql.to_string();
        let mut args = args.clone();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, true);
        }

        info!("[rbatis] [{}] Exec ==> {}", tx_id, &sql);
        info!("[rbatis] [{}] Args ==> {}", tx_id, serde_json::to_string(&args).unwrap_or("".to_string()));
        let result;
        if tx_id.is_empty() {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let mut conn = self.get_pool()?.acquire().await?;
            result = conn.execute_parperd(q).await;
        } else {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let conn = self.tx_context.get_mut(tx_id);
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            result = conn.execute_parperd(q).await;
        }
        if result.is_ok() {
            info!("[rbatis] [{}] RowsAffected <== {}", tx_id, result.as_ref().unwrap());
        } else {
            info!("[rbatis] [{}] RowsAffected <== {}", tx_id, 0);
        }
        return result;
    }


    fn py_to_sql(&self, py: &str, arg: &serde_json::Value) -> Result<(String, Vec<serde_json::Value>), rbatis_core::Error> {
        let nodes = Py::parse_and_cache(py)?;
        let mut arg_array = vec![];
        let mut env = arg.clone();
        let driver_type = self.driver_type()?;
        let mut sql = do_child_nodes(&driver_type, &nodes, &mut env, &self.engine, &mut arg_array)?;
        sql = sql.trim().to_string();
        return Ok((sql, arg_array));
    }

    fn xml_to_sql(&self, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<(String, Vec<serde_json::Value>), rbatis_core::Error> {
        let x = self.mapper_node_map.get(mapper);
        let x = x.to_result(|| format!("[rabtis] mapper:'{}' not load into rbatis", mapper))?;
        let node_type = x.get(method);
        let node_type = node_type.to_result(|| format!("[rabtis] mapper:'{}.{}()' not load into rbatis", mapper, method))?;
        let mut arg_array = vec![];

        let driver_type = self.driver_type()?;
        let mut sql = node_type.eval(&driver_type, &mut arg.clone(), &self.engine, &mut arg_array)?;
        sql = sql.trim().to_string();
        return Ok((sql, arg_array));
    }

    /// fetch result(prepare sql)
    pub async fn xml_fetch<T>(&self, tx_id: &str, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let (sql, args) = self.xml_to_sql(mapper, method, arg)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// exec sql(prepare sql)
    pub async fn xml_exec(&self, tx_id: &str, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<u64, rbatis_core::Error> {
        let (sql, args) = self.xml_to_sql(mapper, method, arg)?;
        return self.exec_prepare(tx_id, sql.as_str(), &args).await;
    }


    /// fetch query result(prepare sql)
    ///for example:
    ///
    ///         let py = r#"
    ///     SELECT * FROM biz_activity
    ///    WHERE delete_flag = #{delete_flag}
    ///     if name != null:
    ///       AND name like #{name+'%'}
    ///     if ids != null:
    ///       AND id in (
    ///       trim ',':
    ///          for item in ids:
    ///            #{item},
    ///       )"#;
    ///         let data: serde_json::Value = rb.py_fetch("", py, &json!({   "delete_flag": 1 })).await.unwrap();
    ///
    pub async fn py_fetch<T>(&self, tx_id: &str, py: &str, arg: &serde_json::Value) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let (sql, args) = self.py_to_sql(py, arg)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// exec sql(prepare sql)
    ///for example:
    ///
    ///         let py = r#"
    ///     SELECT * FROM biz_activity
    ///    WHERE delete_flag = #{delete_flag}
    ///     if name != null:
    ///       AND name like #{name+'%'}
    ///     if ids != null:
    ///       AND id in (
    ///       trim ',':
    ///          for item in ids:
    ///            #{item},
    ///       )"#;
    ///         let data: u64 = rb.py_exec("", py, &json!({   "delete_flag": 1 })).await.unwrap();
    ///
    pub async fn py_exec(&self, tx_id: &str, py: &str, arg: &serde_json::Value) -> Result<u64, rbatis_core::Error> {
        let (sql, args) = self.py_to_sql(py, arg)?;
        return self.exec_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// fetch page result(prepare sql)
    pub async fn fetch_page<T>(&self, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>, page: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync {
        let mut page_result = Page::new(page.get_current(), page.get_size());
        let (count_sql, sql) = self.page_plugin.create_page_sql(&self.driver_type()?, tx_id, sql, args, page)?;
        if page.is_serch_count() {
            //make count sql
            let total: Option<u64> = self.fetch_prepare(tx_id, count_sql.as_str(), args).await?;
            page_result.set_total(total.unwrap_or(0));
            page_result.pages = page_result.get_pages();
            if page_result.get_total() == 0 {
                return Ok(page_result);
            }
        }
        let data: Option<Vec<T>> = self.fetch_prepare(tx_id, sql.as_str(), args).await?;
        page_result.set_records(data.unwrap_or(vec![]));
        page_result.pages = page_result.get_pages();
        return Ok(page_result);
    }


    /// fetch result(prepare sql)
    pub async fn xml_fetch_page<T>(&self, tx_id: &str, mapper: &str, method: &str, arg: &serde_json::Value, page: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync {
        let (sql, args) = self.xml_to_sql(mapper, method, arg)?;
        return self.fetch_page::<T>(tx_id, sql.as_str(), &args, page).await;
    }

    /// fetch result(prepare sql)
    pub async fn py_fetch_page<T>(&self, tx_id: &str, py: &str, arg: &serde_json::Value, page: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync {
        let (sql, args) = self.py_to_sql(py, arg)?;
        return self.fetch_page::<T>(tx_id, sql.as_str(), &args, page).await;
    }
}