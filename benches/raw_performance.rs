#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(private_in_public)]
#![feature(test)]
extern crate test;

use futures_core::future::BoxFuture;
use rbatis::rbatis::RBatis;
use rbatis::{impl_insert, impl_select};
use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, Row};
use rbdc::{block_on, Error};
use rbs::Value;
use std::any::Any;
use test::Bencher;

//cargo test --release --package rbatis --bench raw_performance bench_raw  --no-fail-fast -- --exact -Z unstable-options --show-output
// ---- bench_raw stdout ----(windows)
//Time: 52.4187ms ,each:524 ns/op
//QPS: 1906435 QPS/s
#[test]
fn bench_raw() {
    let f = async {
        let rbatis = RBatis::new();
        rbatis.init(MockDriver {}, "mock://");
        rbatis.acquire().await.unwrap();
        rbatis::bench!(100000, {
            let v = rbatis.query_decode::<Vec<i32>>("", vec![]).await;
        });
    };
    block_on!(f);
}

//cargo test --release --package rbatis --bench raw_performance bench_insert  --no-fail-fast -- --exact -Z unstable-options --show-output
//---- bench_insert stdout ----(macos,cpu-M1Max)
// use Time: 380.693416ms ,each:3806 ns/op
// use QPS: 262676 QPS/s
#[test]
fn bench_insert() {
    let f = async {
        let rbatis = RBatis::new();
        rbatis.init(MockDriver {}, "mock://").unwrap();
        rbatis.acquire().await.unwrap();
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
            create_time: Some(rbdc::datetime::DateTime::now()),
            version: Some(1),
            delete_flag: Some(1),
        };
        rbatis::bench!(100000, {
            MockTable::insert(&mut rbatis.clone(), &t).await.unwrap();
        });
    };
    block_on!(f);
}

//cargo test --release --color=always --package rbatis --bench raw_performance bench_select --no-fail-fast --  --exact -Z unstable-options --show-output
// ---- bench_select stdout ----
// Time: 112.927916ms ,each:1129 ns/op
// QPS: 885486 QPS/s
#[test]
fn bench_select() {
    let f = async {
        let rbatis = RBatis::new();
        rbatis.init(MockDriver {}, "mock://").unwrap();
        rbatis.acquire().await.unwrap();
        rbatis::bench!(100000, {
            MockTable::select_all(&mut rbatis.clone()).await.unwrap();
        });
    };
    block_on!(f);
}

#[derive(Debug, Clone)]
struct MockDriver {}

impl Driver for MockDriver {
    fn name(&self) -> &str {
        "test"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
    }

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(MockConnectOptions {})
    }
}

#[derive(Clone, Debug)]
struct MockConnection {}

impl Connection for MockConnection {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        Box::pin(async { Ok(vec![]) })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        Box::pin(async {
            Ok(ExecResult {
                rows_affected: 0,
                last_insert_id: Value::Null,
            })
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async { Ok(()) })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Clone, Debug)]
struct MockConnectOptions {}

impl ConnectOptions for MockConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
    }

    fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
        Ok(())
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
    pub create_time: Option<rbdc::datetime::DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
impl_insert!(MockTable {});
impl_select!(MockTable {});
