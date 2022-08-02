pub mod encode;
pub mod decode;
pub mod driver;

use std::any::Any;
use std::sync::Arc;
use futures_core::future::BoxFuture;
use futures_util::StreamExt;
use rbdc::db::{Connection, ConnectOptions, MetaData, ExecResult, Row};
use tiberius::{Client, Config, AuthMethod, Column, QueryStream, Query, ColumnData};
use rbdc::{block_on, Error};
use rbs::Value;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use rbs::value::change_lifetime_const;
use crate::decode::Decode;
use crate::encode::Encode;


pub struct MssqlConnection {
    inner: Client<Compat<TcpStream>>,
}

impl MssqlConnection {
    /// let cfg = Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_owned()))?;
    pub async fn establish(cfg: &Config) -> Result<Self, Error> {
        // let cfg = Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_owned()))?;
        let tcp = TcpStream::connect(cfg.get_addr()).await.map_err(|e| Error::from(e.to_string()))?;
        tcp.set_nodelay(true).unwrap();
        let c = Client::connect(cfg.clone(), tcp.compat_write()).await.map_err(|e| Error::from(e.to_string()))?;
        Ok(Self {
            inner: c,
        })
    }
}

#[derive(Debug)]
pub struct MssqlConnectOptions(Config);

impl ConnectOptions for MssqlConnectOptions{
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let v = MssqlConnection::establish(&self.0)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(v) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, url: &str) -> Result<(), Error> {
        *self = MssqlConnectOptions(Config::from_jdbc_string(url).map_err(|e| Error::from(e.to_string()))?);
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}



#[derive(Debug)]
pub struct MssqlRow {
    inner: tiberius::Row,
}

#[derive(Debug)]
pub struct MssqlMetaData {
    inner: &'static [Column],
}

impl MetaData for MssqlMetaData {
    fn column_len(&self) -> usize {
        self.inner.len()
    }

    fn column_name(&self, i: usize) -> String {
        self.inner[i].name().to_string()
    }

    fn column_type(&self, i: usize) -> String {
        format!("{:?}", self.inner[i].column_type())
    }
}

impl Row for MssqlRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        let columns = self.inner.columns();
        Box::new(MssqlMetaData {
            inner: unsafe { change_lifetime_const(columns) }
        })
    }

    fn get(&mut self, i: usize) -> Option<Value> {
        Some(Value::decode(&self.inner, i, self.inner.columns()[i].column_type()))
    }
}


impl Connection for MssqlConnection {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut q = Query::new(sql);
            for x in params {
                x.encode(&mut q)?;
            }
            let v = q.query(&mut self.inner).await.map_err(|e| Error::from(e.to_string()))?;
            let mut results = Vec::with_capacity(0);
            let mut s = v.into_row_stream();
            for item in s.next().await {
                match item {
                    Ok(v) => {
                        results.push(Box::new(MssqlRow {
                            inner: v
                        }) as Box<dyn Row>);
                    }
                    Err(_) => { break; }
                }
            }
            Ok(results)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut q = Query::new(sql);
            for x in params {
                x.encode(&mut q)?;
            }
            let v = q.execute(&mut self.inner).await.map_err(|e| Error::from(e.to_string()))?;
            Ok(ExecResult {
                rows_affected: v.rows_affected().len() as u64,
                last_insert_id: Value::Null,
            })
        })
    }

    fn close(&mut self) -> BoxFuture<'static, Result<(), rbdc::Error>> {
        Box::pin(async move {
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), rbdc::Error>> {
        Box::pin(async move {
            self.inner.execute("ping", &[]).await.map_err(|e| Error::from(e.to_string()))?;
            Ok(())
        })
    }
}

#[test]
fn test() {
    let task = async move {
        let cfg = Config::new();
        let tcp = tokio::net::TcpStream::connect(cfg.get_addr()).await.unwrap();
        tcp.set_nodelay(true).unwrap();
        let c = Client::connect(cfg, tcp.compat_write()).await.unwrap();
    };
    block_on!(task);
}