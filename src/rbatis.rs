use std::cell::Cell;
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Number;
use rbatis_core::db_adapter::{DBPool, DBPoolConn, DBQuery, DBTx, DBExecResult};
use rbatis_core::Error;
use rbatis_core::sync::sync_map::SyncMap;

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
use crate::plugin::log::{LogPlugin, RbatisLog};
use crate::plugin::logic_delete::{LogicDelete, RbatisLogicDeletePlugin};
use crate::plugin::page::{IPage, IPageRequest, Page, PagePlugin, RbatisPagePlugin};
use crate::sql::PageLimit;
use crate::utils::error_util::ToResult;
use crate::wrapper::Wrapper;
use rbatis_core::db::{DriverType, PoolOptions};

/// rbatis engine
pub struct Rbatis {
    // the connection pool,use OnceCell init this
    pub pool: OnceCell<DBPool>,
    // the engine run some express for example:'1+1'=2
    pub engine: RbatisEngine,
    // map<mapper_name,map<method_name,NodeType>>
    pub mapper_node_map: HashMap<String, HashMap<String, NodeType>>,
    //context of tx
    pub tx_context: SyncMap<String, DBTx>,
    // page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    // sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    // logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
    // log plugin
    pub log_plugin: Box<dyn LogPlugin>,
}

impl Default for Rbatis {
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
            tx_context: SyncMap::new(),
            page_plugin: Box::new(RbatisPagePlugin {}),
            sql_intercepts: vec![],
            logic_plugin: None,
            log_plugin: Box::new(RbatisLog {}),
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
        let conn:DBTx = self.get_pool()?.begin().await?;
        //send tx to context
        self.tx_context.insert(new_tx_id.to_string(), conn).await;
        self.log_plugin.info(&format!("[rbatis] [{}] Begin", new_tx_id));
        return Ok(1);
    }

    // /// begin tx,with an exist conn
    // pub async fn begin_with_conn(&self, new_tx_id: &str, mut db_conn: DBPoolConn) -> Result<u64, rbatis_core::Error> {
    //     if new_tx_id.is_empty() {
    //         return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty"));
    //     }
    //     let conn = db_conn.begin().await?;
    //     //send tx to context
    //     self.tx_context.insert(new_tx_id.to_string(), conn).await;
    //     self.log_plugin.info(&format!("[rbatis] [{}] Begin", new_tx_id));
    //     return Ok(1);
    // }

    /// commit tx,and return conn
    pub async fn commit(&self, tx_id: &str) -> Result<(), rbatis_core::Error> {
        let tx = self.tx_context.remove(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let mut tx = tx.unwrap();
        let result = tx.commit().await?;
        self.log_plugin.info(&format!("[rbatis] [{}] Commit", tx_id));
        return Ok(result);
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, tx_id: &str) -> Result<(), rbatis_core::Error> {
        let tx_op = self.tx_context.remove(tx_id).await;
        if tx_op.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let tx = tx_op.unwrap();
        let result = tx.rollback().await?;
        self.log_plugin.info(&format!("[rbatis] [{}] Rollback", tx_id));
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

        self.log_plugin.info(&format!("[rbatis] [{}] Query ==> {}", tx_id, sql.as_str()));
        let result;
        let mut fetch_num = 0;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let (data, num) = conn.fetch(sql.as_str()).await?;
            result = data;
            fetch_num = num;
        } else {
            let conn = self.tx_context.get_mut(tx_id).await;
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            let (data, num) = conn.fetch(sql.as_str()).await?;
            result = data;
            fetch_num = num;
        }
        self.log_plugin.info(&format!("[rbatis] [{}] ReturnRows <== {}", tx_id, fetch_num));
        return Ok(result);
    }

    /// exec sql(row sql)
    pub async fn exec(&self, tx_id: &str, sql: &str) -> Result<DBExecResult, rbatis_core::Error> {

        //sql intercept
        let mut sql = sql.to_string();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut vec![], false);
        }

        self.log_plugin.info(&format!("[rbatis] [{}] Exec ==> :{}", tx_id, &sql));
        let data;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            data = conn.execute(&sql).await?;
        } else {
            let conn = self.tx_context.get_mut(tx_id).await;
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            data = conn.execute(&sql).await?;
        }
        self.log_plugin.info(&format!("[rbatis] [{}] RowsAffected <== {}", tx_id, &data.rows_affected));
        return Ok(data);
    }

    fn bind_arg<'arg>(&self, sql: &'arg str, arg: &Vec<serde_json::Value>) -> Result<DBQuery<'arg>, rbatis_core::Error> {
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

        self.log_plugin.info(&format!("[rbatis] [{}] Query ==> {}", tx_id, &sql));
        self.log_plugin.info(&format!("[rbatis] [{}] Args  ==> {}", tx_id, serde_json::to_string(&args).unwrap_or("".to_string())));
        let result_data;
        let mut return_num = 0;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let (result, num) = conn.fetch_parperd(q).await?;
            result_data = result;
            return_num = num;
        } else {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let conn = self.tx_context.get_mut(tx_id).await;
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            let (result, num) = conn.fetch_parperd(q).await?;
            result_data = result;
            return_num = num;
        }
        self.log_plugin.info(&format!("[rbatis] [{}] ReturnRows <== {}", tx_id, return_num));
        return Ok(result_data);
    }

    /// exec sql(prepare sql)
    pub async fn exec_prepare(&self, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, rbatis_core::Error> {

        //sql intercept
        let mut sql = sql.to_string();
        let mut args = args.clone();
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, true);
        }

        self.log_plugin.info(&format!("[rbatis] [{}] Exec ==> {}", tx_id, &sql));
        self.log_plugin.info(&format!("[rbatis] [{}] Args ==> {}", tx_id, serde_json::to_string(&args).unwrap_or("".to_string())));
        let result;
        if tx_id.is_empty() {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let mut conn = self.get_pool()?.acquire().await?;
            result = conn.exec_prepare(q).await;
        } else {
            let q: DBQuery = self.bind_arg(&sql, &args)?;
            let conn = self.tx_context.get_mut(tx_id).await;
            if conn.is_none() {
                return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
            }
            let mut conn = conn.unwrap();
            result = conn.exec_prepare(q).await;
        }
        if result.is_ok() {
            self.log_plugin.info(&format!("[rbatis] [{}] RowsAffected <== {:#?}", tx_id, result.as_ref()));
        } else {
            self.log_plugin.info(&format!("[rbatis] [{}] RowsAffected <== {}", tx_id, 0));
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
    pub async fn xml_fetch<T, Ser>(&self, tx_id: &str, mapper: &str, method: &str, arg: &Ser) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned, Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.xml_to_sql(mapper, method, &json)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// exec sql(prepare sql)
    pub async fn xml_exec<Ser>(&self, tx_id: &str, mapper: &str, method: &str, arg: &Ser) -> Result<DBExecResult, rbatis_core::Error>
        where Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.xml_to_sql(mapper, method, &json)?;
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
    pub async fn py_fetch<T, Ser>(&self, tx_id: &str, py: &str, arg: &Ser) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned,
              Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.py_to_sql(py, &json)?;
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
    pub async fn py_exec<Ser>(&self, tx_id: &str, py: &str, arg: &Ser) -> Result<DBExecResult, rbatis_core::Error>
        where Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.py_to_sql(py, &json)?;
        return self.exec_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// fetch page result(prepare sql)
    pub async fn fetch_page<T>(&self, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>, page_request: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync {
        let mut page_result = Page::new(page_request.get_current(), page_request.get_size());
        let (count_sql, sql) = self.page_plugin.make_page_sql(&self.driver_type()?, tx_id, sql, args, page_request)?;
        if page_request.is_serch_count() {
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
    pub async fn xml_fetch_page<T, Ser>(&self, tx_id: &str, mapper: &str, method: &str, arg: &Ser, page: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync, Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.xml_to_sql(mapper, method, &json)?;
        return self.fetch_page::<T>(tx_id, sql.as_str(), &args, page).await;
    }

    /// fetch result(prepare sql)
    pub async fn py_fetch_page<T, Ser>(&self, tx_id: &str, py: &str, arg: &Ser, page: &dyn IPageRequest) -> Result<Page<T>, rbatis_core::Error>
        where T: DeserializeOwned + Serialize + Send + Sync,
              Ser: Serialize + Send + Sync {
        let json = serde_json::to_value(arg).unwrap_or(serde_json::Value::Null);
        let (sql, args) = self.py_to_sql(py, &json)?;
        return self.fetch_page::<T>(tx_id, sql.as_str(), &args, page).await;
    }
}