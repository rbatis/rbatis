use crate::rbatis::{Rbatis};
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
use crate::session_factory::{ConnPoolSessionFactory, SessionFactory};
use crate::tx::propagation::Propagation;
use crate::utils::{driver_util, rdbc_util};
use crate::utils::rdbc_util::to_rdbc_values;
use crate::utils::xml_loader::load_xml;



/**
tokio async await 支持
*/
impl Rbatis{

    pub async fn async_raw_sql<T>(id: &str, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let _id=id.to_string();
        let s = eval_sql.to_string();
        return to_tokio_await!(T,{ Rbatis::singleton().raw_sql(&_id,s.as_str())  });
    }

    pub async fn async_begin<T>(id: &str, propagation_type: Propagation) -> Result<String, RbatisError> {
        let _id = id.to_string();
        let data = task::spawn_blocking(move || {
            let data = Rbatis::singleton().begin(_id.as_str(), propagation_type);
            return data;
        }).await;
        if data.is_ok() {
            return data.ok().unwrap();
        } else {
            return Err(RbatisError::from(data.err().unwrap().description()));
        }
    }

    pub async fn async_commit<T>(id: &str) -> Result<String, RbatisError> {
        let _id = id.to_string();
        let data = task::spawn_blocking(move || {
            let data = Rbatis::singleton().commit(_id.as_str());
            return data;
        }).await;
        if data.is_ok() {
            return data.ok().unwrap();
        } else {
            return Err(RbatisError::from(data.err().unwrap().description()));
        }
    }

    pub async fn async_rollback<T>(id: &str) -> Result<String, RbatisError> {
        let _id = id.to_string();
        let data = task::spawn_blocking(move || {
            let data = Rbatis::singleton().rollback(_id.as_str());
            return data;
        }).await;
        if data.is_ok() {
            return data.ok().unwrap();
        } else {
            return Err(RbatisError::from(data.err().unwrap().description()));
        }
    }

    pub async fn async_py_sql<T>(id: &str, env: &Value, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let _id = id.to_string();
        let _env = env.clone();
        let sql = eval_sql.to_string();
        return to_tokio_await!(T,{ Rbatis::singleton().py_sql(_id.as_str(),&_env,&sql)  });
    }


    pub async fn async_mapper<T>(id: &str, mapper_name: &str,mapper_id: &str, env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let _id = id.to_string();
        let _mapper_name = mapper_name.to_string();
        let _mapper_id = mapper_id.to_string();
        let _env = env.clone();
        return to_tokio_await!(T,{ Rbatis::singleton().mapper(&_id,&_mapper_name,&_mapper_id,&_env)  });
    }

    pub async fn async_delete<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let mapper_id = _mapper_id.to_string();
        let env = _env.clone();
        return to_tokio_await!(T,{ Rbatis::singleton().delete(&id,&mapper_name,&env)  });
    }

    pub async fn async_insert<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let mapper_id = _mapper_id.to_string();
        let env = _env.clone();
        return to_tokio_await!(T,{ Rbatis::singleton().insert(&id,&mapper_name,&env)  });
    }

    pub async fn async_update<T>(_id: &str, _mapper_name: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let env = _env.clone();
        return to_tokio_await!(T,{ Rbatis::singleton().update(&id,&mapper_name,&env)  });
    }

    pub async fn async_select<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value) -> Result<T, RbatisError> where T: de::DeserializeOwned + Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let mapper_id = _mapper_id.to_string();
        let env = _env.clone();
        return to_tokio_await!(T,{ Rbatis::singleton().select(&id,&mapper_name,&env)  });
    }


    pub async fn async_select_page<T>(_id: &str, _mapper_name: &str, _env: &Value,_ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: de::DeserializeOwned +Serialize + Clone+ Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let env = _env.clone();
        let ipage=_ipage.clone();
        let data = task::spawn_blocking(move || {
            let data = Rbatis::singleton().select_page(&id,&mapper_name,&env,&ipage);
            return data;
        }).await;
        if data.is_ok() {
            return data.ok().unwrap();
        } else {
            return Err(RbatisError::from(data.err().unwrap().description()));
        }
    }


    pub async fn async_select_page_by_mapper<T>(_id: &str, _mapper_name: &str,_mapper_id: &str, _env: &Value,_ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: de::DeserializeOwned +Serialize + Clone+ Send + 'static {
        let id = _id.to_string();
        let mapper_name = _mapper_name.to_string();
        let mapper_id = _mapper_id.to_string();
        let env = _env.clone();
        let ipage=_ipage.clone();
        let data = task::spawn_blocking(move || {
            let data = Rbatis::singleton().select_page(&id,&mapper_name,&env,&ipage);
            return data;
        }).await;
        if data.is_ok() {
            return data.ok().unwrap();
        } else {
            return Err(RbatisError::from(data.err().unwrap().description()));
        }
    }
}