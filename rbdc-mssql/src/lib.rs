pub extern crate tiberius;

pub mod decode;
pub mod driver;
pub mod encode;

use crate::decode::Decode;
use crate::driver::MssqlDriver;
use crate::encode::Encode;
use futures_core::future::BoxFuture;
use futures_core::Stream;
use rbdc::db::{ConnectOptions, Connection, ExecResult, MetaData, Placeholder, Row};
use rbdc::Error;
use rbs::Value;
use std::sync::Arc;
use tiberius::{Client, Column, ColumnData, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct MssqlConnection {
    inner: Option<Client<Compat<TcpStream>>>,
}

impl MssqlConnection {
    /// let cfg = Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_owned()))?;
    pub async fn establish(cfg: &Config) -> Result<Self, Error> {
        // let cfg = Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_owned()))?;
        let tcp = TcpStream::connect(cfg.get_addr())
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        tcp.set_nodelay(true).unwrap();
        let c = Client::connect(cfg.clone(), tcp.compat_write())
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(Self { inner: Some(c) })
    }
}

#[derive(Debug)]
pub struct MssqlConnectOptions(pub Config);

impl ConnectOptions for MssqlConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let v = MssqlConnection::establish(&self.0)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(v) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, url: &str) -> Result<(), Error> {
        *self = MssqlConnectOptions(
            Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_string()))?,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub struct MssqlRow {
    pub columns: Arc<Vec<Column>>,
    pub datas: Vec<ColumnData<'static>>,
}

#[derive(Debug)]
pub struct MssqlMetaData(pub Arc<Vec<Column>>);

impl MetaData for MssqlMetaData {
    fn column_len(&self) -> usize {
        self.0.len()
    }

    fn column_name(&self, i: usize) -> String {
        self.0[i].name().to_string()
    }

    fn column_type(&self, i: usize) -> String {
        format!("{:?}", self.0[i].column_type())
    }
}

impl Row for MssqlRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(MssqlMetaData(self.columns.clone()))
    }

    fn get(&mut self, i: usize) -> Result<Value, Error> {
        Value::decode(&self.datas[i])
    }
}

impl Connection for MssqlConnection {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        let sql = MssqlDriver {}.exchange(sql);
        Box::pin(async move {
            let mut q = Query::new(sql);
            for x in params {
                x.encode(&mut q)?;
            }
            let v = q
                .query(
                    self.inner
                        .as_mut()
                        .ok_or_else(|| Error::from("MssqlConnection is close"))?,
                )
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            let mut results = Vec::with_capacity(v.size_hint().0);
            let s = v
                .into_results()
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            for item in s {
                for r in item {
                    let mut columns = Vec::with_capacity(r.columns().len());
                    let mut row = MssqlRow {
                        columns: Arc::new(vec![]),
                        datas: Vec::with_capacity(r.columns().len()),
                    };
                    for x in r.columns() {
                        columns.push(x.clone());
                    }
                    row.columns = Arc::new(columns);
                    for x in r {
                        row.datas.push(x);
                    }
                    results.push(Box::new(row) as Box<dyn Row>);
                }
            }
            Ok(results)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        let sql = MssqlDriver {}.exchange(sql);
        Box::pin(async move {
            let mut q = Query::new(sql);
            for x in params {
                x.encode(&mut q)?;
            }
            let v = q
                .execute(
                    self.inner
                        .as_mut()
                        .ok_or_else(|| Error::from("MssqlConnection is close"))?,
                )
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(ExecResult {
                rows_affected: {
                    let mut rows_affected = 0;
                    for x in v.rows_affected() {
                        rows_affected += x.clone();
                    }
                    rows_affected
                },
                last_insert_id: Value::Null,
            })
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move {
            //inner must be Option,so we can take owner and call close(self) method.
            if let Some(v) = self.inner.take() {
                v.close().await.map_err(|e| Error::from(e.to_string()))?;
            }
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), rbdc::Error>> {
        //TODO While 'select 1' can temporarily solve the problem of checking that the connection is valid, it looks ugly.Better replace it with something better way
        Box::pin(async move {
            self.inner
                .as_mut()
                .expect("MssqlConnection inner is none")
                .query("select 1", &[])
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(())
        })
    }
}
