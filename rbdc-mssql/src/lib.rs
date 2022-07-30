pub mod encode;

use futures_core::future::BoxFuture;
use futures_util::StreamExt;
use rbdc::db::{Connection, MetaData, Row};
use tiberius::{Client, Config, AuthMethod};
use rbdc::{block_on, Error};
use rbs::Value;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use crate::encode::Encode;


pub struct MssqlConnection {
    inner: Client<Compat<TcpStream>>,
}

#[derive(Debug)]
pub struct MssqlRow{
    inner:tiberius::Row
}

impl Row for MssqlRow{
    fn meta_data(&self) -> Box<dyn MetaData> {
       todo!()
    }

    fn get(&mut self, i: usize) -> Option<Value> {
        todo!()
    }
}

impl Connection for MssqlConnection {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut args = Vec::with_capacity(params.len());
            for x in &params {
                args.push(x.encode()?);
            }
            let v = self.inner.query(&sql, &args).await.map_err(|e| Error::from(e.to_string()))?;
            let mut results = Vec::with_capacity(0);
            let mut s =v.into_row_stream();
            for item in s.next().await{
                match item{
                    Ok(v) => {
                        results.push(Box::new(MssqlRow{
                            inner:v
                        }) as Box<dyn Row>);
                    }
                    Err(_) => {break;}
                }
            }
            Ok(results)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, rbdc::Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let mut args = Vec::with_capacity(params.len());
            for x in &params {
                args.push(x.encode()?);
            }
            let v = self.inner.execute(&sql, &args).await.map_err(|e| Error::from(e.to_string()))?;
            Ok(v.rows_affected().len() as u64)
        })
    }

    fn close(&mut self) -> BoxFuture<'static, Result<(), rbdc::Error>> {
        Box::pin(async move{
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), rbdc::Error>> {
        Box::pin(async move{
            self.inner.execute("ping",&[]).await.map_err(|e| Error::from(e.to_string()))?;
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