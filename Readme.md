[WebSite](https://rbatis.github.io/rbatis.io) | [Showcase](https://github.com/rbatis/rbatis/network/dependents) | [Example](https://github.com/rbatis/example)

[![Build Status](https://github.com/rbatis/rbatis/workflows/ci/badge.svg)](https://github.com/zhuxiujia/rbatis/actions)
[![doc.rs](https://docs.rs/rbatis/badge.svg)](https://docs.rs/rbatis/)
[![](https://img.shields.io/crates/d/rbatis)](https://crates.io/crates/rbatis)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![GitHub release](https://img.shields.io/github/v/release/rbatis/rbatis)](https://github.com/rbatis/rbatis/releases)
[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

<img style="width: 200px;height: 200px;" width="200" height="200" src="logo.png" />

#### a compile-time code generation ORM that balances ease of writing with performance and robustness

It is an ORM, a small compiler, a dynamic SQL languages

* High-performance: Compile time [Dynamic SQL](dyn_sql.md),Based on Future/Tokio, Connection Pool
* Reliability:  Rust Safe Code,precompile: `#{arg}`, Direct replacement:`${arg}`, unify `?` placeholders(support all driver)
* Productivity: Powerful [Interceptor interface](https://github.com/rbatis/rbatis/blob/master/src/plugin/intercept_log.rs),  `#[py_sql]` ,  `#[html_sql] (just like mybatis3)`, `curd!`,  [RBDC driver](https://crates.io/crates/rbdc), [rbdc-mysql](https://crates.io/crates/rbdc-mysql),[rbdc-pg](https://crates.io/crates/rbdc-pg),[rbdc-sqlite](https://crates.io/crates/rbdc-sqlite),[Table synchronize plugin](https://rbatis.github.io/rbatis.io/#/v4/?id=plugin-table-sync),[abs_admin project](https://github.com/rbatis/abs_admin)
* maintainability: The RBDC driver supports custom drivers, custom connection pool,support third-party driver package

###### Thanks to ```SQLX, deadpool,mobc, Tiberius, MyBatis, xorm``` and so on reference design or code implementation. Release of V4 is Inspired and supported by these frameworks.**



### Performance

* this bench test is MockTable,MockDriver,MockConnection to Assume that the network I/O time is 0
* run code ```rbatis.query_decode::<Vec<i32>>("", vec![]).await;``` on benches bench_raw()
* run code ```MockTable::insert(&rbatis,&t).await;``` on benches bench_insert()
* run code ```MockTable::select_all(&rbatis).await.unwrap();``` on benches bench_select()

```
---- bench_raw stdout ----(windows)
Time: 52.4187ms ,each:524 ns/op
QPS: 1906435 QPS/s

---- bench_insert stdout ----(macos,cpu-M1Max)
Time: 378.186333ms ,each:3781 ns/op
QPS: 264418 QPS/s

---- bench_select stdout ----(macos,cpu-M1Max)
Time: 112.927916ms ,each:1129 ns/op
QPS: 885486 QPS/s
```

### Supported data structures

| data structure                                                        | is supported |
|-----------------------------------------------------------------------|--------------|
| Option                                                                | √            | 
| Vec                                                                   | √            |  
| HashMap                                                               | √            |
| i32,i64,f32,f64,bool,String...more rust type                          | √            |  
| rbatis::rbdc::types::{Date,DateTime,Time,Timestamp,Decimal,Json}      | √            |
| rbatis::plugin::page::{Page<T>, PageRequest}                          | √            |
| rbs::Value*                                                           | √            |
| serde_json::*                                                         | √            |
| any serde type                                                        | √            |
| driver type on package (rdbc-mysql/types,rbdc-pg/types,rbdc-sqlite/types) | √            |

### Supported database driver

| database(crates.io)                                 | github_link                                                                    |
|-----------------------------------------------------|--------------------------------------------------------------------------------|
| [Mysql](https://crates.io/crates/rbdc-mysql)        | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [Postgres](https://crates.io/crates/rbdc-pg)        | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg)         |
| [Sqlite](https://crates.io/crates/rbdc-sqlite)      | [rbatis/rbdc-sqlite](https://github.com/rbatis/rbatis/tree/master/rbdc-sqlite) |
| [Mssql](https://crates.io/crates/rbdc-mssql)        | [rbatis/rbdc-mssql](https://github.com/rbatis/rbatis/tree/master/rbdc-mssql)   |
| [MariaDB](https://crates.io/crates/rbdc-mysql)      | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [TiDB](https://crates.io/crates/rbdc-mysql)         | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql)   |
| [CockroachDB](https://crates.io/crates/rbdc-pg)     | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg)         |
| [Oracle](https://crates.io/crates/rbdc-oracle)      | [chenpengfan/rbdc-oracle](https://github.com/chenpengfan/rbdc-oracle)          |
| [TDengine](https://crates.io/crates/rbdc-tdengine)  | [tdcare/rbdc-tdengine](https://github.com/tdcare/rbdc-tdengine)                |


### Supported OS/Platforms by [Workflows CI](https://github.com/rbatis/rbatis/actions)

| platform                | is supported |
|-------------------------|--------------|
| Linux(unbutu laster***) | √            | 
| Apple/MacOS(laster)     | √            |  
| Windows(latest)         | √            |

### Supported Web Frameworks

* any web Frameworks just like ntex, actix-web, axum, hyper, rocket, tide, warp, salvo and more.

##### Quick example: QueryWrapper and common usages (see example/crud_test.rs for details)

* Rust v1.73+ later

* Cargo.toml

#### default
```toml
#rbatis deps
rbs = { version = "4.5"}
rbatis = { version = "4.5"}
rbdc-sqlite = { version = "4.5" }
#rbdc-mysql={version="4.5"}
#rbdc-pg={version="4.5"}
#rbdc-mssql={version="4.5"}

#other deps
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```
#### (option) 'native-tls'
```toml
rbs = { version = "4.5" }
rbdc-sqlite = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
#rbdc-mysql={version="4.5", default-features = false, features = ["tls-native-tls"]}
#rbdc-pg={version="4.5", default-features = false, features = ["tls-native-tls"]}
#rbdc-mssql={version="4.5", default-features = false, features = ["tls-native-tls"]}
rbatis = { version = "4.5", default-features = false, features = ["tls-native-tls","default_mode"] }
#other deps
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

#### default use
```rust
//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate rbdc;
use rbatis::rbdc::datetime::DateTime;

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
    pub create_time: Option<DateTime>,
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
    let rb = RBatis::new();
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
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let data = BizActivity::insert(&rb, &activity).await;
    println!("insert = {:?}", data);

    let data = BizActivity::select_all_by_id(&rb, "1", "1").await;
    println!("select_all_by_id = {:?}", data);

    let data = BizActivity::select_by_id(&rb, "1".to_string()).await;
    println!("select_by_id = {:?}", data);

    let data = BizActivity::update_by_column(&rb, &activity, "id").await;
    println!("update_by_column = {:?}", data);

    let data = BizActivity::update_by_name(&rb, &activity, "test").await;
    println!("update_by_name = {:?}", data);

    let data = BizActivity::delete_by_column(&rb, "id", &"2".into()).await;
    println!("delete_by_column = {:?}", data);

    let data = BizActivity::delete_by_name(&rb, "2").await;
    println!("delete_by_column = {:?}", data);

    let data = BizActivity::select_page(&rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page = {:?}", data);
}
///...more usage,see crud.rs
```

* raw-sql
```rust
#[tokio::main]
pub async fn main() {
    use rbatis::RBatis;
    use rbdc_sqlite::driver::SqliteDriver;
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
    }
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    let table: Option<BizActivity> = rb
        .query_decode("select * from biz_activity limit ?", vec![rbs::to_value!(1)])
        .await
        .unwrap();
    let count: u64 = rb
        .query_decode("select count(1) as count from biz_activity", vec![])
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
async fn py_sql_tx(rb: &RBatis, tx_id: &String, name: &str) -> Vec<BizActivity> { impled!() }
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
async fn select_by_condition(rb: &dyn Executor, page_req: &PageRequest, name: &str) -> Page<BizActivity> { impled!() }
```

```rust
use once_cell::sync::Lazy;

pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

/// Macro generates execution logic based on method definition, similar to @select dynamic SQL of Java/Mybatis
/// RB is the name referenced locally by RBatis, for example DAO ::RB, com:: XXX ::RB... Can be
/// The second parameter is the standard driver SQL. Note that the corresponding database parameter mysql is? , pg is $1...
/// macro auto edit method to  'pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}'
///
#[sql("select * from biz_activity where id = ?")]
pub async fn select(rb: &RBatis, name: &str) -> BizActivity {}
//or： pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}

#[tokio::test]
pub async fn test_macro() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
    let a = select(&RB, "1").await.unwrap();
    println!("{:?}", a);
}
```

# How it works

Rely on rbatis-codegen to create the source code of the corresponding structure from the html file at compile time (with debug_mode(Cargo.toml- ``` rbatis = { features = ["debug_mode"]} ```) enabled, you can observe the code-generated function), and call the generated method directly at run time.
We know that compilation is generally divided into three steps, lexes, syntactic analysis, semantic analysis, and intermediate code generation. In rbatis,
Lexical analysis is handled by the dependent func.rs in `rbatis-codegen`, which relies on syn and quote.
Parsing is done by parser_html and parser_pysql in `rbatis-codegen`
The generated syntax tree is a structure defined in the syntax_tree package in `rbatis-codegen`
Intermediate code generation has func.rs generation function, all supported functions are defined in `rbatis-codegen`

What is described above occurs during the cargo build phase, which is the compilation phase of the rust procedural macro, where the code generated by `rbatis-codegen` is handed back to the rust compiler for LLVM compilation to produce pure machine code


So I think rbatis is Truly zero overhead dynamic SQL compile-time ORM.

# Submit PR(Pull Requests)

You are welcome to submit the merge, and make sure that any functionality you add has the appropriate mock unit test function added under the test package.


# [Changelog](https://github.com/rbatis/rbatis/releases/)

# Roadmap

- [x] sqlite table sync plugin(auto create table/column)
- [x] customize connection pooling,connection pool add more dynamically configured parameters
- [ ] More simplified syntax

# Contact/donation, or click on star [rbatis](https://github.com/rbatis/rbatis)

* [![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

# 联系方式/捐赠,或 [rbatis](https://github.com/rbatis/rbatis) 点star

> 捐赠

<img style="width: 200px;height: 300px;" width="200" height="300" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> 联系方式(添加好友请备注'rbatis') 微信群：先加微信，然后拉进群

<img style="width: 200px;height: 250px;" width="200" height="250" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />


