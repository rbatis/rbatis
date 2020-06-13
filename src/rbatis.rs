use crate::ast::lang::py::Py;

use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::ast::lang::xml::Xml;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::update_node::UpdateNode;
use crate::ast::node::select_node::SelectNode;
use rbatis_core::mysql::{MySqlPool, MySql};
use rbatis_core::executor::Executor;
use serde::de::DeserializeOwned;
use rbatis_core::cursor::Cursor;
use crate::ast::ast::RbatisAST;
use rbatis_core::query::{query, Query};
use rbatis_core::query_as::query_as;

/// rbatis engine
pub struct Rbatis<'r> {
    pool: Option<MySqlPool>,
    engine: RbatisEngine,
    /// map<mapper_name,map<method_name,NodeType>>
    mapper_node_map: HashMap<&'r str, HashMap<String, NodeType>>,
}


impl<'r> Rbatis<'r> {
    pub async fn new(url: &str) -> Rbatis<'r> {
        let mut pool = Option::None;
        if url.ne("") {
            pool = Some(MySqlPool::new(url).await.unwrap());
        }
        return Rbatis { pool, mapper_node_map: HashMap::new(), engine: RbatisEngine::new() };
    }

    pub fn load_xml(&mut self, mapper_name: &'r str, data: &str) -> Result<(), rbatis_core::Error> {
        let xml = Xml::parser(data);
        self.mapper_node_map.insert(mapper_name, xml);
        return Ok(());
    }

    /// fetch result(row sql)
    pub async fn fetch<T>(&self, sql: &str) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        let mut c = conn.fetch(sql);
        return c.decode().await;
    }

    /// exec sql(row sql)
    pub async fn exec(&self, sql: &str) -> Result<u64, rbatis_core::Error> {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        return conn.execute(sql).await;
    }

    /// fetch result(prepare sql)
    pub async fn fetch_prepare<T>(&self, sql: &str, arg: &Vec<serde_json::Value>) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        let mut q: Query<MySql> = query(sql);
        for x in arg {
            q = q.bind(x.to_string());
        }
        let mut c = conn.fetch(q);
        return c.decode().await;
    }

    /// exec sql(prepare sql)
    pub async fn exec_prepare(&self, sql: &str, arg: &Vec<serde_json::Value>) -> Result<u64, rbatis_core::Error> {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        unimplemented!()
    }


    /// fetch result(prepare sql)
    pub async fn xml_fetch<T>(&self, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let x = self.mapper_node_map.get(mapper).unwrap();
        let node_type = x.get(method).unwrap();
        let mut arg_array = vec![];
        let sql = node_type.eval(&mut arg.clone(), &self.engine, &mut arg_array).unwrap();
        unimplemented!()
    }

    /// exec sql(prepare sql)
    pub async fn xml_exec(&self, mapper: &str, method: &str, arg: &serde_json::Value) -> Result<u64, rbatis_core::Error> {
        unimplemented!()
    }

    /// fetch result(prepare sql)
    pub async fn py_fetch<T>(&self,  py: &str, arg: &serde_json::Value) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        unimplemented!()
    }

    /// exec sql(prepare sql)
    pub async fn py_exec(&self, py: &str, arg: &serde_json::Value) -> Result<u64, rbatis_core::Error> {
        unimplemented!()
    }

}