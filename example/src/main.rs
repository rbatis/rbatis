#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;

use chrono::NaiveDateTime;
use fast_log::filter::ModuleFilter;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tide::Request;

use rbatis::crud::CRUDEnable;
use rbatis::rbatis::Rbatis;

mod crud_test;

///数据库表模型,支持BigDecimal ,DateTime ,rust基本类型（int,float,uint,string,Vec,Array）
/// CRUDEnable 特性会自动识别 id为表的id类型(识别String)，自动识别结构体名称为蛇形命名的表名 biz_activity。没有id的表 请手动指定
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

// (可选) 手动实现，不使用上面的derive(CRUDEnable),可重写table_name方法。手动实现能支持IDE智能提示
// impl CRUDEnable for BizActivity {
//     type IdType = String;
// }

//示例 mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  static ref RB:Rbatis=Rbatis::new();
}


//启动web服务，并且对表执行 count统计
#[async_std::main]
async fn main() {
    fast_log::init_log("requests.log",
                       1000,
                       log::Level::Info,
                       Some(Box::new(ModuleFilter::new_exclude(vec!["sqlx".to_string(), "tide".to_string()]))),
                       true);
    RB.link(MYSQL_URL).await.unwrap();
    let mut app = tide::new();
    app.at("/").get(|_: Request<()>| async move {
        // println!("accept req[{} /test] arg: {:?}",req.url().to_string(),a);
        let arg = &vec![json!(1)];
        let v = RB.fetch_prepare("", "SELECT count(1) FROM biz_activity where delete_flag = ?;", arg).await;
        if v.is_ok() {
            let data: Value = v.unwrap();
            Ok(data.to_string())
        } else {
            Ok(v.err().unwrap().to_string())
        }
    });
    //app.at("/").get(|_| async { Ok("Hello, world!") });
    let addr = "0.0.0.0:8000";
    println!("http server listen on http://localhost:8000");
    app.listen(addr).await.unwrap();
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use log::info;
    use serde_json::json;
    use serde_json::Value;

    use rbatis::ast::ast::RbatisAST;
    use rbatis::ast::node::proxy_node::{ProxyNode, CustomNodeGenerate};
    use rbatis::ast::node::node_type::NodeType;
    use rbatis::core::db::{DriverType, PoolOptions};
    use rbatis::core::db_adapter::DBPool;
    use rbatis::core::Error;
    use rbatis::crud::CRUD;
    use rbatis::crud::CRUDEnable;
    use rbatis::engine::runtime::RbatisEngine;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;
    use rbatis::utils::bencher::QPS;

    use crate::BizActivity;

    pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

    // 示例-Rbatis示例初始化(必须)
    lazy_static! {
      static ref RB:Rbatis=Rbatis::new();
   }

    ///测试打印表名称
    #[test]
    pub fn test_table_info() {
        println!("table_name: {}", BizActivity::table_name());
    }

    // 示例-打印日志
    #[test]
    pub fn test_log() {
        //1 启用日志(可选，不添加则不加载日志库)
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        info!("print data");
        sleep(Duration::from_secs(1));
    }

    //示例-Rbatis直接使用驱动
    #[async_std::test]
    pub async fn test_use_driver() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let pool = DBPool::new(MYSQL_URL).await.unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let (r, _) = conn.fetch::<serde_json::Value>("SELECT count(1) FROM biz_activity;").await.unwrap();
        println!("done:{:?}", r);
    }

    //示例-Rbatis直接使用驱动+Wrapper
    #[async_std::test]
    pub async fn test_use_driver_wrapper() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();

        let name = "test";
        let w = rb.new_wrapper()
            .push_sql("SELECT count(1) FROM biz_activity WHERE ")
            .r#in("delete_flag", &[0, 1])
            .and()
            .ne("delete_flag", -1)
            .do_if(!name.is_empty(), |w| w.and().like("name", name))
            .check().unwrap();
        let (r, _): (serde_json::Value, rbatis::core::Error) = rb.fetch_prepare_wrapper("", &w).await.unwrap();
        println!("done:{:?}", r);
    }

    //示例-Rbatis直接使用驱动-prepared stmt sql
    #[async_std::test]
    pub async fn test_prepare_sql() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let arg = &vec![json!(1), json!("test%")];
        let r: Vec<BizActivity> = rb.fetch_prepare("", "SELECT * FROM biz_activity WHERE delete_flag =  ? AND name like ?", arg).await.unwrap();
        println!("done:{}", serde_json::to_string(&r).unwrap_or(String::new()));
    }


    //示例-Rbatis使用py风格的语法查询
    #[async_std::test]
    pub async fn test_py_sql() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let py = r#"
    SELECT * FROM biz_activity
    WHERE delete_flag = #{delete_flag}
    if name != null:
      AND name like #{name+'%'}
    if ids != null:
      AND id in (
      trim ',':
         for item in ids:
           #{item},
      )"#;
        let data: serde_json::Value = rb.py_fetch("", py, &json!({   "delete_flag": 1 })).await.unwrap();
        println!("{}", data);
    }

    //示例-Rbatis语法分页
    #[async_std::test]
    pub async fn test_sql_page() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let wraper = rb.new_wrapper()
            .eq("delete_flag", 0).check().unwrap();
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper, &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    //示例-Rbatis使用py风格的语法分页
    #[async_std::test]
    pub async fn test_py_sql_page() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let py = r#"
    SELECT * FROM biz_activity
    WHERE delete_flag = #{delete_flag}
    if name != null:
      AND name like #{name+'%'}"#;
        let data: Page<BizActivity> = rb.py_fetch_page("", py, &json!({   "delete_flag": 1 }), &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }


    #[derive(Debug)]
    pub struct MyNode {}

    impl RbatisAST for MyNode {
        fn name() -> &'static str {
            "custom"
        }

        fn eval(&self, convert: &DriverType, env: &mut Value, engine: &RbatisEngine, arg_result: &mut Vec<Value>) -> Result<String, Error> {
            Ok(" AND id = 1 ".to_string())
        }
    }

    pub struct MyGen {}

    impl CustomNodeGenerate for MyGen {
        fn generate(&self, express: &str, child_nodes: Vec<NodeType>) -> Result<Option<ProxyNode>, Error> {
            if express.starts_with(MyNode::name()) {
                return Ok(Option::from(ProxyNode::from(MyNode {}, child_nodes)));
            }
            //skip
            return Ok(None);
        }
    }

    //示例-Rbatis扩展py风格的语法
    #[async_std::test]
    pub async fn test_py_sql_custom() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let mut rb = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        rb.py.add_gen(MyGen {});
        let py = "
    SELECT * FROM biz_activity
    WHERE delete_flag = 0
    custom :
    ";
        let data: Page<BizActivity> = rb.py_fetch_page("", py, &json!({}), &PageRequest::new(1, 20)).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    }


    //示例-Rbatis使用事务
    #[async_std::test]
    pub async fn test_tx() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        let rb: Rbatis = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let tx_id = "1";
        rb.begin(tx_id).await.unwrap();
        let v: serde_json::Value = rb.fetch(tx_id, "SELECT count(1) FROM biz_activity;").await.unwrap();
        println!("{}", v.clone());
        rb.commit(tx_id).await.unwrap();
    }


    //示例-Rbatis使用事务
    #[async_std::test]
    pub async fn test_tx_manager() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);

        let rb: Rbatis = Rbatis::new();
        rb.link(MYSQL_URL).await.unwrap();
        let tx_id = "1";
        rb.begin(tx_id).await.unwrap();
        let v: serde_json::Value = rb.fetch(tx_id, "SELECT count(1) FROM biz_activity;").await.unwrap();
        println!("{}", v.clone());
        drop(rb);
        sleep(Duration::from_secs(10));
    }

    /// 示例-Rbatis使用web框架Tide、async_std
    #[async_std::test]
    pub async fn test_tide() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        RB.link(MYSQL_URL).await.unwrap();
        let mut app = tide::new();
        app.at("/").get(|_: tide::Request<()>| async move {
            // println!("accept req[{} /test] arg: {:?}",req.url().to_string(),a);
            let v = RB.fetch("", "SELECT count(1) FROM biz_activity;").await;
            if v.is_ok() {
                let data: Value = v.unwrap();
                Ok(data.to_string())
            } else {
                Ok(v.err().unwrap().to_string())
            }
        });
        //app.at("/").get(|_| async { Ok("Hello, world!") });
        let addr = "0.0.0.0:8000";
        println!("server on http://{}", addr);
        app.listen(addr).await.unwrap();
    }


    async fn hello(_: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Infallible> {
        let v = RB.fetch("", "SELECT count(1) FROM biz_activity;").await;
        if v.is_ok() {
            let data: Value = v.unwrap();
            Ok(hyper::Response::new(hyper::Body::from(data.to_string())))
        } else {
            Ok(hyper::Response::new(hyper::Body::from(v.err().unwrap().to_string())))
        }
    }

    // 示例-hyper /AsyncStd(async std也依赖了tokio,所以它也支持~)
    #[async_std::test]
    //#[tokio::test] //也可以使用tokio::test
    pub async fn test_hyper() {
        fast_log::init_log("requests.log",
                           1000,
                           log::Level::Info,
                           None,
                           true);
        RB.link(MYSQL_URL).await.unwrap();
        let make_svc = hyper::service::make_service_fn(|_conn| {
            async { Ok::<_, Infallible>(hyper::service::service_fn(hello)) }
        });
        let addr = ([0, 0, 0, 0], 8000).into();
        let server = hyper::Server::bind(&addr).serve(make_svc);
        println!("Listening on http://{}", addr);
        server.await.unwrap();
    }

    //golang                       : BenchmarkQps-12   1297457 ns/op
    //rust(30% fast with golang)   : 975.3122ms   ,each:975312 ns/op
    //cargo test --release --color=always --package example --bin example test::bench_qps --no-fail-fast -- --exact -Z unstable-options --show-output
    #[async_std::test]
    pub async fn bench_qps() {
        let mut opts = PoolOptions::default();
        //test_before_acquire = false will improve performance
        opts.test_before_acquire = true;
        RB.link_opt(MYSQL_URL, &opts).await.unwrap();
        let total = 10000;
        let mut current = 0;
        let now = std::time::Instant::now();
        loop {
            let v: rbatis::core::Result<serde_json::Value> = RB.fetch("", "select count(1) from biz_activity WHERE delete_flag = 1;").await;
            if current == total - 1 {
                let end = SystemTime::now();
                now.time(total);
                now.qps(total);
                break;
            } else {
                current = current + 1;
            }
        }
    }
}