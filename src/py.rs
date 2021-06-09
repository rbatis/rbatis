use crate::rbatis::Rbatis;
use crate::core::db::{DBExecResult, DBPool, DBPoolConn, DBPoolOptions, DBQuery, DBTx, DriverType};
use crate::core::Error;
use serde::de::DeserializeOwned;
use crate::utils::string_util;
use serde::Serialize;
use crate::plugin::page::{IPageRequest, Page, IPage};
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor, ExecutorMut, RbatisRef};
use async_trait::async_trait;
use crate::crud::CRUDMut;

#[async_trait]
pub trait PySqlConvert: RbatisRef {
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
}

impl<'a> PySqlConvert for RBatisConnExecutor<'a> {}

impl<'a> PySqlConvert for RBatisTxExecutor<'a> {}

impl PySqlConvert for Rbatis{

}