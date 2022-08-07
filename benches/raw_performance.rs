#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use std::any::Any;
use test::Bencher;
use futures_core::future::BoxFuture;
use rbatis::impl_insert;
use rbatis::rbatis::Rbatis;
use rbdc::db::{Connection, ConnectOptions, Driver, ExecResult, Row};
use rbdc::{block_on, Error};
use rbdc::rt::block_on;
use rbs::Value;


//cargo test --release --package rbatis --bench raw_performance bench_raw  --no-fail-fast -- --exact -Z unstable-options --show-output

//---- bench_raw stdout----(win10,cpu-amd5950x)
// use Time: 464.2µs ,each:46 ns/op
// use QPS: 20533880 QPS/s
#[test]
fn bench_raw() {
    let rbatis = block_on(async {
        let rbatis = Rbatis::new();
        rbatis.link(MockDriver {}, "").await;
        rbatis
    });
    block_on!(async{
        rbatis::bench!(100000,{
            let v=rbatis.fetch::<Vec<i32>>("",vec![]).await;
        });
   });
}


//cargo test --release --package rbatis --bench raw_performance bench_insert  --no-fail-fast -- --exact -Z unstable-options --show-output
//---- bench_insert stdout ----(win10,cpu-amd5950x)
// use Time: 130.5443ms ,each:1305 ns/op
// use QPS: 765860 QPS/s
#[test]
fn bench_insert() {
    let rbatis = block_on(async {
        let rbatis = Rbatis::new();
        rbatis.link(MockDriver {}, "").await;
        rbatis
    });
    block_on!(async{
        let mut t = MockTable {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(rbdc::datetime::FastDateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
        rbatis::bench!(100000,{
            MockTable::insert(&mut rbatis.clone(),&t).await;
        });
   });
}


#[derive(Debug, Clone)]
pub struct MockDriver {}

impl Driver for MockDriver {
    fn name(&self) -> &str {
        "test"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async {
            Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async {
            Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(MockConnectOptions {})
    }
}

#[derive(Clone, Debug)]
pub struct MockConnection {}

impl Connection for MockConnection {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        Box::pin(async {
            Ok(vec![])
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        Box::pin(async {
            Ok(ExecResult {
                rows_affected: 0,
                last_insert_id: Value::Null,
            })
        })
    }

    fn close(&mut self) -> BoxFuture< Result<(), Error>> {
        Box::pin(async {
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async {
            Ok(())
        })
    }
}

#[derive(Clone, Debug)]
pub struct MockConnectOptions {}

impl ConnectOptions for MockConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async {
            Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct MockTable {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<rbdc::datetime::FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
impl_insert!(MockTable {});