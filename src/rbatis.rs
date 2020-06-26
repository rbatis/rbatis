use std::cell::Cell;
use std::collections::HashMap;

use log::{error, info, LevelFilter, warn};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;

use rbatis_core::connection::Connection;
use rbatis_core::cursor::Cursor;
use rbatis_core::db::{DBPool, DBPoolConn, DBQuery, DBTx, DBType};
use rbatis_core::Error;
use rbatis_core::executor::Executor;
use rbatis_core::pool::{Pool, PoolConnection};
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
    pool: OnceCell<DBPool>,
    engine: RbatisEngine,
    /// map<mapper_name,map<method_name,NodeType>>
    mapper_node_map: HashMap<&'r str, HashMap<String, NodeType>>,
    context_tx: SyncMap<DBTx>,
}


impl<'r> Rbatis<'r> {
    pub fn new() -> Rbatis<'static> {
        return Rbatis { pool: OnceCell::new(), mapper_node_map: HashMap::new(), engine: RbatisEngine::new(), context_tx: SyncMap::new() };
    }

    pub fn check(&self) {
        println!("self.pool: {:?}", self.pool);
        println!("self.mapper_node_map: {:?}", self.mapper_node_map);
    }

    /// link pool
    pub async fn link(&self, url: &str) -> Result<(), rbatis_core::Error> {
        if url.ne("") {
            let pool = DBPool::new(url).await?;
            self.pool.get_or_init(|| {
                pool
            });
        }
        return Ok(());
    }

    pub fn load_xml(&mut self, mapper_name: &'r str, data: &str) -> Result<(), rbatis_core::Error> {
        let xml = Xml::parser(data);
        self.mapper_node_map.insert(mapper_name, xml);
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

    async fn get_tx(&self, tx_id: &str) -> Result<DBTx, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        return Ok(tx.unwrap());
    }

    /// begin tx,for new conn
    pub async fn begin(&self, tx_id: &str) -> Result<u64, rbatis_core::Error> {
        if tx_id.is_empty() {
            return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty"));
        }
        let conn = self.get_pool()?.begin().await?;
        //send tx to context
        self.context_tx.put(tx_id, conn).await;
        return Ok(1);
    }

    /// commit tx,and return conn
    pub async fn commit(&self, tx_id: &str) -> Result<DBPoolConn, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let mut tx = tx.unwrap();
        let result = tx.commit().await?;
        return Ok(result);
    }

    /// rollback tx,and return conn
    pub async fn rollback(&self, tx_id: &str) -> Result<DBPoolConn, rbatis_core::Error> {
        let tx = self.context_tx.pop(tx_id).await;
        if tx.is_none() {
            return Err(rbatis_core::Error::from(format!("[rbatis] tx:{} not exist！", tx_id)));
        }
        let mut tx = tx.unwrap();
        let result = tx.rollback().await?;
        return Ok(result);
    }


    /// fetch result(row sql)
    pub async fn fetch<T>(&self, tx_id: &str, sql: &str) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        info!("[rbatis] >> fetch sql:{}", sql);
        let data;
        let fetch_num;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let mut c = conn.fetch(sql)?;
            let json = c.fetch_json().await?;
            fetch_num = json.len();
            data = rbatis_core::decode::json_decode::<T>(json)?;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            //now conn must return to context
            let c = conn.fetch(sql);
            if c.is_err() {
                let e = c.err().unwrap();
                //send tx back to context
                self.context_tx.put(tx_id, conn).await;
                return Err(e);
            }
            let mut c = c.unwrap();
            let json = c.fetch_json().await;
            if json.is_err() {
                let e = json.err().unwrap();
                //send tx back to context
                self.context_tx.put(tx_id, conn).await;
                return Err(e);
            }
            //send tx back to context
            self.context_tx.put(tx_id, conn).await;
            let json = json.unwrap();
            fetch_num = json.len();
            data = rbatis_core::decode::json_decode::<T>(json)?;
        }
        info!("[rbatis] << {}", fetch_num);
        return Ok(data);
    }

    /// exec sql(row sql)
    pub async fn exec(&self, tx_id: &str, sql: &str) -> Result<u64, rbatis_core::Error> {
        info!("[rbatis] >> exec sql:{}", sql);
        let data;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            data = conn.execute(sql).await?;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let result = conn.execute(sql).await;
            //send tx back to context
            self.context_tx.put(tx_id, conn).await;
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            data = result.unwrap();
        }
        info!("[rbatis] << {}", &data);
        return Ok(data);
    }

    fn bind_arg<'a>(&self, sql: &'a str, arg: &Vec<serde_json::Value>) -> DBQuery<'a> {
        let mut q: DBQuery = self.pool.get().unwrap().make_query(sql).unwrap();
        for x in arg {
            match x {
                serde_json::Value::String(s) => {
                    q.bind(s);
                }
                _ => {
                    q.bind(x.to_string().as_str());
                }
            }
        }
        return q;
    }

    /// fetch result(prepare sql)
    pub async fn fetch_prepare<T>(&self, tx_id: &str, sql: &str, arg: &Vec<serde_json::Value>) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        info!("[rbatis] >> fetch sql:{}", sql);
        info!("[rbatis] >> fetch arg:{}", serde_json::to_string(arg).unwrap_or("".to_string()));
        let result;
        let return_num;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let q: DBQuery = self.bind_arg(sql, arg);
            let mut c = conn.fetch_parperd(q)?;
            let json_array = c.fetch_json().await?;
            return_num = json_array.len();
            result = rbatis_core::decode::json_decode::<T>(json_array)?;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            //now conn use finish must be return to context
            let q: DBQuery = self.bind_arg(sql, arg);
            let c = conn.fetch_parperd(q);
            if c.is_err() {
                let e = c.err().unwrap();
                //send tx back to context
                self.context_tx.put(tx_id, conn).await;
                return Err(e);
            }
            let mut c = c.unwrap();
            let json = c.fetch_json().await;
            if json.is_err() {
                let e = json.err().unwrap();
                //send tx back to context
                self.context_tx.put(tx_id, conn).await;
                return Err(e);
            }
            //send tx back to context
            self.context_tx.put(tx_id, conn).await;
            let json = json.unwrap();
            return_num = json.len();
            result = rbatis_core::decode::json_decode::<T>(json)?;
        }
        info!("[rbatis] << {}", return_num);
        return Ok(result);
    }

    /// exec sql(prepare sql)
    pub async fn exec_prepare(&self, tx_id: &str, sql: &str, arg: &Vec<serde_json::Value>) -> Result<u64, rbatis_core::Error> {
        info!("[rbatis] >> exec sql:{}", sql);
        info!("[rbatis] >> exec arg:{}", serde_json::to_string(arg).unwrap_or("".to_string()));
        let result;
        if tx_id.is_empty() {
            let mut conn = self.get_pool()?.acquire().await?;
            let q: DBQuery = self.bind_arg(sql, arg);
            result = conn.execute_parperd(q).await;
        } else {
            let mut conn = self.get_tx(tx_id).await?;
            let q: DBQuery = self.bind_arg(sql, arg);
            result = conn.execute_parperd(q).await;
            //send tx back to context
            self.context_tx.put(tx_id, conn).await;
        }
        if result.is_ok() {
            info!("[rbatis] << {}", result.as_ref().unwrap());
        } else {
            info!("[rbatis] << {}", 0);
        }
        return result;
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