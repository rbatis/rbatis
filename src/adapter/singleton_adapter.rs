use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::process::exit;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use log4rs::init_file;
use serde::{de, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{Number, Value};
use serde_json::json;
use serde_json::ser::State::Rest;
use tokio::task;
use uuid::Uuid;

use crate::ast::ast::Ast;
use crate::ast::lang::py::Py;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::node::{do_child_nodes, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::ast::node::string_node::StringNode;
use crate::crud::ipage::IPage;
use crate::db_config::DBConfig;
use crate::decode::rdbc_driver_decoder::decode_result_set;
use crate::engine::runtime::RbatisEngine;
use crate::error::RbatisError;
use crate::local_session::LocalSession;
use crate::rbatis::Rbatis;
use crate::session_factory::{ConnPoolSessionFactory, SessionFactory};
use crate::tx::propagation::Propagation;
use crate::utils::{driver_util, rdbc_util};
use crate::utils::rdbc_util::to_rdbc_values;
use crate::utils::xml_loader::load_xml;

impl Rbatis {
    
    pub fn singleton_raw_sql<T>(id: &str, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        Rbatis::singleton().raw_sql(id, eval_sql)
    }

    pub fn singleton_begin<T>(id: &str, propagation_type: Propagation) -> Result<String, RbatisError> {
        return Rbatis::singleton().begin(id,propagation_type);
    }

    pub fn singleton_commit<T>(id: &str) -> Result<String, RbatisError> {
        return Rbatis::singleton().commit(id);
    }

    pub fn singleton_rollback<T>(id: &str) -> Result<String, RbatisError> {
        return Rbatis::singleton().rollback(id);
    }

    pub fn singleton_py_sql<T>(id: &str, env: &Value, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().py_sql(id,env,eval_sql);
    }


    pub fn singleton_mapper<T>(id: &str, mapper_name: &str,mapper_id: &str, env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().mapper(id,mapper_name,mapper_id,env);
    }

    pub fn singleton_delete<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().delete(_id,_mapper_name,_env);
    }

    pub fn singleton_insert<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().insert(_id,_mapper_name,_env);
    }

    pub fn singleton_update<T>(_id: &str, _mapper_name: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().update(_id,_mapper_name,_env);
    }

    pub fn singleton_select<T>(id: &str, mapper_name: &str, env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned  {
        return Rbatis::singleton().select(id,mapper_name,env);
    }


    pub fn singleton_select_page<T>(id: &str, mapper_name: &str, arg: &Value, ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: de::DeserializeOwned +Serialize + Clone {
        return Rbatis::singleton().select_page(id,mapper_name,arg,ipage);
    }


    pub fn singleton_select_page_by_mapper<T>(id: &str, mapper_name: &str, mapper_id: &str, env: &Value, ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: de::DeserializeOwned +Serialize + Clone {
        return Rbatis::singleton().select_page_by_mapper(id,mapper_name,mapper_id,env,ipage);
    }
}