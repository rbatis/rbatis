use std::collections::HashMap;

use serde::de::DeserializeOwned;

use rbatis_core::connection::Connection;
use rbatis_core::cursor::Cursor;
use rbatis_core::Error;
use rbatis_core::executor::Executor;
use rbatis_core::mysql::{MySql, MySqlConnection, MySqlPool};
use rbatis_core::pool::PoolConnection;
use rbatis_core::query::{query, Query};
use rbatis_core::query_as::query_as;
use rbatis_core::sync_map::SyncMap;
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
use crate::utils::error_util::ToResult;

/// rbatis engine
pub struct Rbatis<'r> {
    pool: Option<MySqlPool>,
    engine: RbatisEngine,
    /// map<mapper_name,map<method_name,NodeType>>
    mapper_node_map: HashMap<&'r str, HashMap<String, NodeType>>,
    context_tx: SyncMap<Transaction<PoolConnection<MySqlConnection>>>,
}


impl<'r> Rbatis<'r> {
    pub async fn new(url: &str) -> Result<Rbatis<'r>, rbatis_core::Error> {
        let mut pool = Option::None;
        if url.ne("") {
            pool = Some(MySqlPool::new(url).await?);
        }
        return Ok(Rbatis { pool, mapper_node_map: HashMap::new(), engine: RbatisEngine::new(), context_tx: SyncMap::new() });
    }

    pub fn load_xml(&mut self, mapper_name: &'r str, data: &str) -> Result<(), rbatis_core::Error> {
        let xml = Xml::parser(data);
        self.mapper_node_map.insert(mapper_name, xml);
        return Ok(());
    }

    /// get conn pool
    pub fn get_pool(&self) -> Result<&MySqlPool, rbatis_core::Error> {
        if self.pool.is_none() {
            return Err(rbatis_core::Error::from("[rbatis] rbatis pool not inited!"));
        }
        return Ok(self.pool.as_ref().unwrap());
    }

    async fn get_tx(&self, tx_id: &str) -> Result<Transaction<PoolConnection<MySqlConnection>>, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        return Ok(tx.unwrap());
    }


    pub async fn begin(&self, tx_id: &str) -> Result<u64, rbatis_core::Error> {
        if tx_id.is_empty() {
            return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn = self.get_pool()?.begin().await?;
        self.context_tx.put(tx_id, conn).await;
        return Ok(1);
    }

    pub async fn commit(&self, tx_id: &str) -> Result<PoolConnection<MySqlConnection>, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let tx = tx.unwrap();
        let result = tx.commit().await?;
        return Ok(result);
    }

    pub async fn rollback(&self, tx_id: &str) -> Result<PoolConnection<MySqlConnection>, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let tx = tx.unwrap();
        let result = tx.rollback().await?;
        return Ok(result);
    }


    /// fetch result(row sql)
    pub async fn fetch<T>(&self, tx_id: &str, sql: &str) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let mut c = conn.fetch(sql);
            return c.decode().await;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let mut c = conn.fetch(sql);
            let t = c.decode().await;
            self.context_tx.put(tx_id, conn).await;
            return t;
        }
    }

    /// exec sql(row sql)
    pub async fn exec(&self, tx_id: &str, sql: &str) -> Result<u64, rbatis_core::Error> {
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            return conn.execute(sql).await;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let result = conn.execute(sql).await;
            self.context_tx.put(tx_id, conn).await;
            return result;
        }
    }

    /// fetch result(prepare sql)
    pub async fn fetch_prepare<T>(&self, tx_id: &str, sql: &str, arg: &Vec<serde_json::Value>) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let mut q: Query<MySql> = query(sql);
            for x in arg {
                q = q.bind(x.to_string());
            }
            let mut c = conn.fetch(q);
            return c.decode().await;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let mut q: Query<MySql> = query(sql);
            for x in arg {
                q = q.bind(x.to_string());
            }
            let mut c = conn.fetch(q);
            let result = c.decode().await;
            self.context_tx.put(tx_id, conn).await;
            return result;
        }
    }

    /// exec sql(prepare sql)
    pub async fn exec_prepare(&self, tx_id: &str, sql: &str, arg: &Vec<serde_json::Value>) -> Result<u64, rbatis_core::Error> {
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let mut q: Query<MySql> = query(sql);
            for x in arg {
                q = q.bind(x.to_string());
            }
            return conn.execute(q).await;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let mut q: Query<MySql> = query(sql);
            for x in arg {
                q = q.bind(x.to_string());
            }
            let result = conn.execute(q).await;
            self.context_tx.put(tx_id, conn).await;
            return result;
        }
    }


    fn xml_to_sql(&self, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<(String, Vec<serde_json::Value>), rbatis_core::Error> {
        let x = self.mapper_node_map.get(mapper);
        let x = x.to_result(|| format!("[rabtis] mapper:{} not init to rbatis", mapper))?;
        let node_type = x.get(method);
        let node_type = node_type.to_result(|| format!("[rabtis] mapper:{}.{}() not init to rbatis", mapper, method))?;
        let mut arg_array = vec![];
        let sql = node_type.eval(&mut arg.clone(), &self.engine, &mut arg_array)?;
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


    fn py_to_sql(&self, py: &str, arg: &serde_json::Value) -> Result<(String, Vec<serde_json::Value>), rbatis_core::Error> {
        let nodes = Py::parser_and_cache(py)?;
        let mut arg_array = vec![];
        let sql = do_child_nodes(&nodes, &mut arg.clone(), &self.engine, &mut arg_array)?;
        return Ok((sql, arg_array));
    }

    /// fetch result(prepare sql)
    pub async fn py_fetch<T>(&self, tx_id: &str, py: &str, arg: &serde_json::Value) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let (sql, args) = self.py_to_sql(py, arg)?;
        return self.fetch_prepare(tx_id, sql.as_str(), &args).await;
    }

    /// exec sql(prepare sql)
    pub async fn py_exec(&self, tx_id: &str, py: &str, arg: &serde_json::Value) -> Result<u64, rbatis_core::Error> {
        let (sql, args) = self.py_to_sql(py, arg)?;
        return self.exec_prepare(tx_id, sql.as_str(), &args).await;
    }
}