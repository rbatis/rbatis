pub mod encode;
pub mod decode;
pub mod driver;

use std::sync::Arc;
use futures_core::future::BoxFuture;
use futures_util::StreamExt;
use rbdc::db::{Connection, MetaData, Row};
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
        Some(Value::decode(&self.inner,i,self.inner.columns()[i].column_type()))
    }
}


impl Connection for MssqlConnection {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut q =Query::new(sql);
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

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut q = Query::new(sql);
            for x in params {
                x.encode(&mut q)?;
            }
            let v = q.execute(&mut self.inner).await.map_err(|e| Error::from(e.to_string()))?;
            Ok(v.rows_affected().len() as u64)
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