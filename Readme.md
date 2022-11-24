[WebSite](https://rbatis.github.io/rbatis.io) | [Showcase](https://github.com/rbatis/rbatis/network/dependents)

[![Build Status](https://github.com/rbatis/rbatis/workflows/ci/badge.svg)](https://github.com/zhuxiujia/rbatis/actions)
[![doc.rs](https://docs.rs/rbatis/badge.svg)](https://docs.rs/rbatis/)
[![](https://img.shields.io/crates/d/rbatis)](https://crates.io/crates/rbatis)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![GitHub release](https://img.shields.io/github/v/release/rbatis/rbatis)](https://github.com/rbatis/rbatis/releases)
[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

<img style="width: 200px;height: 200px;" width="200" height="200" src="logo.png" />

#### A async, pure Rust SQL Toolkit,compile-time Dynamic SQL,Compile Time ORM. 

It is an ORM, a small compiler, a dynamic SQL languages

* Non-invasive API design.You don't need to change the current code
* Compatible with most mybatis3 syntax
* No Runtimes，No Garbage Collection,High performance, Based on Future/Tokio
* Zero cost [Dynamic SQL](dyn_sql.md), implemented using (proc-macro,compile-time,Cow(Reduce unnecessary cloning))
  techniques。 don't need ONGL engine(mybatis)
* JDBC-like driver design, driver use cargo.toml dependency and ```Box<dyn Driver>``` separation
* All database drivers supported ```#{arg}```, ```${arg}```,```?```  placeholder(pg/mssql auto processing '?' to '$1'
  and '@P1')
* Dynamic SQL(Write code freely in SQL),pagination, ```py_sql``` query lang and ```html_sql```(Inspired Mybatis).
* Dynamic configuration connection pool(Based on the deadpool)
* Supports logging, customizable logging based on `log` crate
* 100% Safe Rust with `#![forbid(unsafe_code)]` enabled
* Support use Trait System Add ```py_sql/ html_sql```
  functions.[see](https://github.com/rbatis/rbatis/blob/master/example/src/macro_proc_htmlsql_custom_func.rs)
* [abs_admin project](https://github.com/rbatis/abs_admin)  an complete background user management system(
  Vue.js+rbatis+actix-web)
  

Thanks to ```SQLX, deadpool, Tiberius, MyBatis,xorm``` and so on reference design or code implementation. release of V4.0
  is Inspired and supported by these frameworks

### Performance

* this bench test is MockTable,MockDriver,MockConnection to Assume that the network I/O time is 0
* run code ```MockTable::insert(&mut rbatis.clone(),&t).await;``` on benches/fn bench_insert()
* * run code ```MockTable::select_all(&mut rbatis.clone()).await.unwrap();``` on benches/fn bench_insert()
* use
  command ``` cargo test --release --package rbatis --bench raw_performance bench_insert --no-fail-fast -- --exact -Z unstable-options --show-output ```

```
//---- bench_insert stdout ----(macos,cpu-M1Max)
//Time: 378.186333ms ,each:3781 ns/op
//QPS: 264418 QPS/s

// ---- bench_select stdout ----(macos,cpu-M1Max)
// Time: 112.927916ms ,each:1129 ns/op
// QPS: 885486 QPS/s
```

### Supported data structures

| data structure                                                            | is supported |
|---------------------------------------------------------------------------|--------------|
| Option                                                                    | √            | 
| Vec                                                                       | √            |  
| HashMap                                                                   | √            |
| i32,i64,f32,f64,bool,String...more rust type                              | √            |  
| rbatis::rbdc::types::{Date,FastDateTime,Time,Timestamp,Decimal,Json}      | √            |
| rbatis::plugin::page::{Page<T>, PageRequest}                              | √            |
| rbs::Value*                                                               | √            |
| serde_json::*                                                             | √            |
| any serde type                                                            | √            |
| driver type on package (rdbc-mysql/types,rbdc-pg/types,rbdc-sqlite/types) | √            |

### Supported database driver

| database(crates.io)                             | github_link                                                                    |
|-------------------------------------------------|--------------------------------------------------------------------------------|
| [Mysql](https://crates.io/crates/rbdc-mysql)    | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [Postgres](https://crates.io/crates/rbdc-pg)    | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg)         |
| [Sqlite](https://crates.io/crates/rbdc-sqlite)  | [rbatis/rbdc-sqlite](https://github.com/rbatis/rbatis/tree/master/rbdc-sqlite) |
| [Mssql](https://crates.io/crates/rbdc-mssql)    | [rbatis/rbdc-mssql](https://github.com/rbatis/rbatis/tree/master/rbdc-mssql)   |
| [MariaDB](https://crates.io/crates/rbdc-mysql)  | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [TiDB](https://crates.io/crates/rbdc-mysql)     | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [CockroachDB](https://crates.io/crates/rbdc-pg) | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg)         |
| [Oracle](https://crates.io/crates/rbdc-oracle)  | [chenpengfan/rbdc-oracle](https://github.com/chenpengfan/rbdc-oracle)          |


### Supported OS/Platforms by [Workflows CI](https://github.com/rbatis/rbatis/actions)

| platform                | is supported |
|-------------------------|--------------|
| Linux(unbutu laster***) | √            | 
| Apple/MacOS(laster)     | √            |  
| Windows(latest)         | √            |

### Supported Web Frameworks

* any web Frameworks just like actix-web,axum,hyper*,rocket,tide,warp,salvo...... and more

##### Quick example: QueryWrapper and common usages (see example/crud_test.rs for details)

* Rust v1.63+ later

* Cargo.toml

```toml
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.5"
serde = { version = "1", features = ["derive"] }
rbs = { version = "0.1"}
rbatis = { version = "4.0"}
rbdc-sqlite = { version = "0.1" }
#rbdc-mysql={version="0.1"}
#rbdc-pg={version="0.1"}
#rbdc-mssql={version="0.1"}
#...and more driver
```

```rust
//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate rbdc;
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
crud!(BizActivity{});//crud = insert+select_by_column+update_by_column+delete_by_column

impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(BizActivity{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_update!(BizActivity{update_by_name(name:&str) => "`where id = 1`"});
impl_delete!(BizActivity {delete_by_name(name:&str) => "`where name= '2'`"});
impl_select_page!(BizActivity{select_page(name:&str) => "`where name != #{name}`"});

#[tokio::main]
async fn main() {
    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let rb = Rbatis::new();
    /// connect to database  
    // sqlite 
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // mysql 
    // rb.init(MysqlDriver{},"mysql://root:123456@localhost:3306/test").unwrap();
    // postgresql 
    // rb.init(PgDriver{},"postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // mssql/sqlserver
    // rb.init(MssqlDriver{},"jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test").unwrap();

    let activity = BizActivity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(FastDateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let data = BizActivity::insert(&mut rb, &activity).await;
    println!("insert = {:?}", data);

    let data = BizActivity::select_all_by_id(&mut rb, "1", "1").await;
    println!("select_all_by_id = {:?}", data);

    let data = BizActivity::select_by_id(&mut rb, "1".to_string()).await;
    println!("select_by_id = {:?}", data);

    let data = BizActivity::update_by_column(&mut rb, &activity, "id").await;
    println!("update_by_column = {:?}", data);

    let data = BizActivity::update_by_name(&mut rb, &activity, "test").await;
    println!("update_by_name = {:?}", data);

    let data = BizActivity::delete_by_column(&mut rb, "id", &"2".into()).await;
    println!("delete_by_column = {:?}", data);

    let data = BizActivity::delete_by_name(&mut rb, "2").await;
    println!("delete_by_column = {:?}", data);

    let data = BizActivity::select_page(&mut rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page = {:?}", data);
}
///...more usage,see crud.rs
```

* raw-sql
```rust
#[tokio::main]
pub async fn main() {
    use rbatis::Rbatis;
    use rbdc_sqlite::driver::SqliteDriver;
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
    }
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = Rbatis::new();
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    let table: Option<BizActivity> = rb
        .fetch_decode("select * from biz_activity limit ?", vec![rbs::to_value!(1)])
        .await
        .unwrap();
    let count: u64 = rb
        .fetch_decode("select count(1) as count from biz_activity", vec![])
        .await
        .unwrap();
    println!(">>>>> table={:?}", table);
    println!(">>>>> count={}", count);
}
```

#### macros

* Important update (pysql removes runtime, directly compiles to static rust code)    This means that the performance of
  SQL generated using py_sql,html_sql is roughly similar to that of handwritten code.

> Because of the compile time, the annotations need to declare the database type to be used.

```rust
    #[py_sql("select * from biz_activity where delete_flag = 0
                  if name != '':
                    `and name=#{name}`")]
async fn py_sql_tx(rb: &Rbatis, tx_id: &String, name: &str) -> Vec<BizActivity> { impled!() }
```

* Added html_sql support, a form of organization similar to MyBatis, to facilitate migration of Java systems to Rust(
  Note that it is also compiled as Rust code at build time and performs close to handwritten code)  this is very faster

> Because of the compile time, the annotations need to declare the database type to be used

```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
        "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <select id="select_by_condition">
        `select * from biz_activity where `
        <if test="name != ''">
            name like #{name}
        </if>
    </select>
</mapper>
```

```rust
    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
#[html_sql("example/example.html")]
async fn select_by_condition(rb: &mut dyn Executor, page_req: &PageRequest, name: &str) -> Page<BizActivity> { impled!() }
```

```rust
use once_cell::sync::Lazy;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

/// Macro generates execution logic based on method definition, similar to @select dynamic SQL of Java/Mybatis
/// RB is the name referenced locally by Rbatis, for example DAO ::RB, com:: XXX ::RB... Can be
/// The second parameter is the standard driver SQL. Note that the corresponding database parameter mysql is? , pg is $1...
/// macro auto edit method to  'pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}'
///
#[sql("select * from biz_activity where id = ?")]
pub async fn select(rb: &Rbatis, name: &str) -> BizActivity {}
//or： pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}

#[tokio::test]
pub async fn test_macro() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
    let a = select(&RB, "1").await.unwrap();
    println!("{:?}", a);
}
```

# [Changelog](https://github.com/rbatis/rbatis/releases/)

# Roadmap

- [x] sqlite table sync plugin(auto create table/column)
- [ ] Static analysis and generate executable test functions

# Contact/donation, or click on star [rbatis](https://github.com/rbatis/rbatis)

* [![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

# 联系方式/捐赠,或 [rbatis](https://github.com/rbatis/rbatis) 点star

> 捐赠

<img style="width: 400px;height: 600px;" width="400" height="600" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> 联系方式(添加好友请备注'rbatis') 微信群：先加微信，然后拉进群

<img style="width: 400px;height: 500px;" width="400" height="500" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />


