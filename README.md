# Rbatis

##### 📖 English Documentation | 📖 [中文文档](README_CN.md)

[Website](https://rbatis.github.io/rbatis.io) | [Showcase](https://github.com/rbatis/rbatis/network/dependents) | [Examples](https://github.com/rbatis/rbatis/tree/master/example)

[![Build Status](https://github.com/rbatis/rbatis/workflows/ci/badge.svg)](https://github.com/zhuxiujia/rbatis/actions)
[![doc.rs](https://docs.rs/rbatis/badge.svg)](https://docs.rs/rbatis/)
[![](https://img.shields.io/crates/d/rbatis)](https://crates.io/crates/rbatis)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![codecov](https://codecov.io/gh/rbatis/rbatis/graph/badge.svg?token=VAVPXSHoff)](https://codecov.io/gh/rbatis/rbatis)
[![GitHub release](https://img.shields.io/github/v/release/rbatis/rbatis)](https://github.com/rbatis/rbatis/releases)
[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

<img style="width: 200px;height: 190px;" width="200" height="190" src="logo.png" />

## Introduction

Rbatis is a high-performance ORM framework for Rust based on compile-time code generation. It perfectly balances development efficiency, performance, and stability, functioning as both an ORM and a dynamic SQL compiler.

## AI-support
* - [rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## Core Advantages

### 1. High Performance
- **🚀 Dynamic SQL to Native Rust Translation**: The key advantage - compiles dynamic SQL directly to native Rust code, achieving performance equivalent to hand-written SQL
- **Zero Runtime Overhead**: All SQL parsing and optimization happens at compile-time, eliminating runtime interpretation costs
- **Based on Tokio Async Model**: Fully utilizes Rust's async features to enhance concurrency performance
- **Efficient Connection Pools**: Built-in multiple connection pool implementations, optimizing database connection management

### 2. Reliability
- **Rust Safety Features**: Leverages Rust's ownership and borrowing checks to ensure memory and thread safety
- **Unified Parameter Placeholders**: Uses `?` as a unified placeholder, supporting all drivers
- **Two Replacement Modes**: Precompiled `#{arg}` and direct replacement `${arg}`, meeting different scenario requirements

### 3. Development Efficiency
- **Powerful ORM Capabilities**: Automatic mapping between database tables and Rust structures
- **Multiple SQL Building Methods**:
  - **py_sql**: Python-style dynamic SQL with `if`, `for`, `choose/when/otherwise`, `bind`, `trim` structures and collection operations (`.sql()`, `.csv()`)
  - **html_sql**: MyBatis-like XML templates with familiar tag structure (`<if>`, `<where>`, `<set>`, `<foreach>`), declarative SQL building, and automatic handling of SQL fragments without requiring CDATA
  - **Raw SQL**: Direct SQL statements
- **CRUD Macros**: Generate common CRUD operations with a single line of code
- **Interceptor Plugin**: [Custom extension functionality](https://rbatis.github.io/rbatis.io/#/v4/?id=plugin-intercept)
- **Table Sync Plugin**: [Automatically create/update table structures](https://rbatis.github.io/rbatis.io/#/v4/?id=plugin-table-sync)

### 4. Extensibility
- **Multiple Database Support**: MySQL, PostgreSQL, SQLite, MSSQL, MariaDB, TiDB, CockroachDB, Oracle, TDengine, etc.
- **Custom Driver Interface**: Implement a simple interface to add support for new databases
- **Multiple Connection Pools**: FastPool (default), Deadpool, MobcPool
- **Compatible with Various Web Frameworks**: Seamlessly integrates with ntex, actix-web, axum, hyper, rocket, tide, warp, salvo, and more

## Supported Database Drivers

| Database (crates.io)                               | GitHub Link                                                                       |
|----------------------------------------------------|-----------------------------------------------------------------------------------|
| [MySQL](https://crates.io/crates/rbdc-mysql)       | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [PostgreSQL](https://crates.io/crates/rbdc-pg)     | [rbatis/rbdc-pg](https://github.com/rbatis/rbdc/tree/master/rbdc-pg)              |
| [SQLite](https://crates.io/crates/rbdc-sqlite)     | [rbatis/rbdc-sqlite](https://github.com/rbatis/rbdc/tree/master/rbdc-sqlite)      |
| [MSSQL](https://crates.io/crates/rbdc-mssql)       | [rbatis/rbdc-mssql](https://github.com/rbatis/rbdc/tree/master/rbdc-mssql)        |
| [Truso](https://crates.io/crates/rbdc-turso)       | [rbatis/rbdc-turso](https://github.com/rbatis/rbdc/tree/master/rbdc-turso)        |
| [MariaDB](https://crates.io/crates/rbdc-mysql)     | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [TiDB](https://crates.io/crates/rbdc-mysql)        | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [CockroachDB](https://crates.io/crates/rbdc-pg)    | [rbatis/rbdc-pg](https://github.com/rbatis/rbdc/tree/master/rbdc-pg)              |
| [Oracle](https://crates.io/crates/rbdc-oracle)     | [chenpengfan/rbdc-oracle](https://github.com/chenpengfan/rbdc-oracle)             |
| [TDengine](https://crates.io/crates/rbdc-tdengine) | [tdcare/rbdc-tdengine](https://github.com/tdcare/rbdc-tdengine)                   |

## Supported Connection Pools

| Connection Pool (crates.io)                               | GitHub Link                                                                       |
|-----------------------------------------------------------|-----------------------------------------------------------------------------------|
| [FastPool (default)](https://crates.io/crates/rbdc-pool-fast) | [rbatis/fast_pool](https://github.com/rbatis/rbatis/tree/master/rbdc-pool-fast) |
| [Deadpool](https://crates.io/crates/rbdc-pool-deadpool)       | [rbatis/rbdc-pool-deadpool](https://github.com/rbatis/rbdc-pool-deadpool)      |
| [MobcPool](https://crates.io/crates/rbdc-pool-mobc)            | [rbatis/rbdc-pool-mobc](https://github.com/rbatis/rbdc-pool-mobc)              |

## Supported Data Types

| Data Type                                                               | Support |
|-------------------------------------------------------------------------|---------|
| `Option`                                                                | ✓       |
| `Vec`                                                                   | ✓       |
| `HashMap`                                                               | ✓       |
| `i32, i64, f32, f64, bool, String`, and other Rust base types           | ✓       |
| `rbatis::rbdc::types::{Bytes, Date, DateTime, Time, Timestamp, Decimal, Json}` | ✓ |
| `rbatis::plugin::page::{Page, PageRequest}`                             | ✓       |
| `rbs::Value`                                                            | ✓       |
| `serde_json::Value` and other serde types                               | ✓       |
| Driver-specific types from rbdc-mysql, rbdc-pg, rbdc-sqlite, rbdc-mssql | ✓       |


## Member crates

| crate                                 | GitHub Link                                     |
|---------------------------------------|-------------------------------------------------|
| [rbdc](https://crates.io/crates/rbdc) | [rbdc](https://github.com/rbatis/rbdc)          |
| [rbs](https://crates.io/crates/rbs)   | [rbs](https://github.com/rbatis/rbs)             |



## How Rbatis Works

Rbatis uses compile-time code generation through the `rbatis-codegen` crate, which means:

1. **🎯 Direct Translation to Native Rust**: Dynamic SQL is converted to optimized Rust code during compilation, achieving performance identical to hand-written SQL without any runtime interpretation overhead.

2. **Compilation Process**:
   - **Lexical Analysis**: Handled by `func.rs` in `rbatis-codegen` using Rust's `syn` and `quote` crates
   - **Syntax Parsing**: Performed by `parser_html` and `parser_pysql` modules in `rbatis-codegen`
   - **Abstract Syntax Tree**: Built using structures defined in the `syntax_tree` package in `rbatis-codegen`
   - **Intermediate Code Generation**: Executed by `func.rs`, which contains all the code generation functions

3. **Build Process Integration**: The entire process runs during the `cargo build` phase as part of Rust's procedural macro compilation. The generated code is returned to the Rust compiler for LLVM compilation to produce machine code.

4. **Compile-Time SQL Optimization**: Unlike traditional ORMs that interpret dynamic SQL at runtime (causing performance penalties), Rbatis translates SQL to native Rust code at compile-time, delivering hand-written SQL performance while maintaining ORM convenience.

## Performance Benchmarks

```
---- bench_raw stdout ----(windows/SingleThread)
Time: 52.4187ms ,each:524 ns/op
QPS: 1906435 QPS/s

---- bench_select stdout ----(macos-M1Cpu/SingleThread)
Time: 112.927916ms ,each:1129 ns/op
QPS: 885486 QPS/s

---- bench_insert stdout ----(macos-M1Cpu/SingleThread)
Time: 346.576666ms ,each:3465 ns/op
QPS: 288531 QPS/s
```

## Quick Start

### Dependencies

```toml
# Cargo.toml
[dependencies]
#drivers
rbdc-sqlite = { version = "4" }
# rbdc-mysql = { version = "4" }
# rbdc-pg = { version = "4" }
# rbdc-mssql = { version = "4" }
rbs = { version = "4"}
rbatis = { version = "4.8"}

# Other dependencies
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

### Basic Usage

```rust
use rbatis::rbdc::datetime::DateTime;
use rbs::value;
use rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_time: Option<DateTime>
}

// Automatically generate CRUD methods
crud!(Activity{});

#[tokio::main]
async fn main() {
    // Configure logging
    fast_log::init(fast_log::Config::new().console()).expect("log init fail");
    
    // Initialize rbatis
    let rb = RBatis::new();
    
    // Connect to database
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").expect("rb init fail");
    // /// other databases
    // rb.init(PgDriver{}, "postgres://postgres:123456@localhost:5432/postgres").expect("pool init fail");
    // rb.init(MysqlDriver{}, "mysql://root:123456@localhost:3306/test").expect("pool init fail");
    
    // Create data
    let activity = Activity {
        id: Some("1".into()),
        name: Some("Test Activity".into()),
        create_time: Some(DateTime::now()),
    };
    let arrays = vec![
        Activity {
            id: Some("2".into()),
            name: Some("Activity 2".into()),
            create_time: Some(DateTime::now())
        },
        Activity {
            id: Some("3".into()),
            name: Some("Activity 3".into()),
            create_time: Some(DateTime::now())
        },
    ];

    // Insert data
    let data = Activity::insert(&rb, &activity).await;

    // Batch insert
    let data = Activity::insert_batch(&rb, &arrays, 10).await;

    // Update by map condition (updates all fields)
    let data = Activity::update_by_map(&rb, &activity, value!{ "id": "1" }).await;

    // Update only specific columns using the "column" key in condition (GitHub issue #591)
    let data = Activity::update_by_map(&rb, &activity, value!{ "id": "1", "column": ["name", "status"] }).await;

    // Query by map condition
    let data = Activity::select_by_map(&rb, value!{"id":"2","name":"Activity 2"}).await;

    // LIKE query
    let data = Activity::select_by_map(&rb, value!{"name like ":"%Activity%"}).await;

    // Greater than query
    let data = Activity::select_by_map(&rb, value!{"id > ":"2"}).await;

    // IN query
    let data = Activity::select_by_map(&rb, value!{"id": &["1", "2", "3"]}).await;

    // Delete by map condition
    let data = Activity::delete_by_map(&rb, value!{"id": &["1", "2", "3"]}).await;
}
```

### Advanced Usage (html_sql)

Use `#[rbatis::html_sql("html/file_path")]` macro for complex queries like pagination, join queries, etc.:

```rust
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>
}
#[rbatis::html_sql("example/example.html")]
impl Activity {
    // Paginated query (PageIntercept handles limit/offset automatically)
    pub async fn select_by_page(rb: &dyn rbatis::Executor, page_req: &rbatis::PageRequest, name: &str) -> rbatis::Result<rbatis::Page<Activity>> {impled!()}
    pub async fn select_by_condition(rb: &dyn rbatis::Executor,name: &str) -> rbatis::Result<Vec<Activity>> {impled!()}
    pub async fn update_by_id(rb: &dyn rbatis::Executor,arg: &Activity) -> rbatis::Result<rbatis::rbdc::ExecResult> {impled!()}
    pub async fn delete_by_id(rb: &dyn rbatis::Executor,id: &str) -> rbatis::Result<rbatis::rbdc::ExecResult> {impled!()}
}
```

Corresponding HTML template file `example/example.html`:
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
 <select id="select_by_page">
    SELECT * FROM activity
    <where>
        <if test="name != ''">
            AND name LIKE #{name}
        </if>
    </where>
 </select>
 <select id="select_by_condition">
    SELECT * FROM activity
    <where>
        <if test="name != ''">
            AND name LIKE #{name}
        </if>
    </where>
 </select>
 <update id="update_by_id">
        ` update activity `
        <set collection="arg"></set>
        ` where id = #{id} `
 </update>
 <delete id="delete_by_id">
    DELETE FROM activity WHERE id = #{id}
 </delete>   
</mapper>
```

**Applicable scenarios**: paginated queries, join queries, complex dynamic SQL, multi-condition search

## Creating a Custom Database Driver

To implement a custom database driver for Rbatis:

1. Define your driver project with dependencies:
```toml
[features]
default = ["tls-rustls"]
tls-rustls=["rbdc/tls-rustls"]
tls-native-tls=["rbdc/tls-native-tls"]
[dependencies]
rbs = { version = "4"}
rbdc = { version = "4", default-features = false, optional = true }
fastdate = { version = "0.3" }
tokio = { version = "1", features = ["full"] }
```

2. Implement the required traits:
```rust
use rbdc::db::{Driver, MetaData, Row, Connection, ConnectOptions, Placeholder};

pub struct YourDriver{}
impl Driver for YourDriver{}

pub struct YourMetaData{}
impl MetaData for YourMetaData{}

pub struct YourRow{}
impl Row for YourRow{}

pub struct YourConnection{}
impl Connection for YourConnection{}

pub struct YourConnectOptions{}
impl ConnectOptions for YourConnectOptions{}

pub struct YourPlaceholder{}
impl Placeholder for YourPlaceholder{}

// Then use your driver:
#[tokio::main]
async fn main() -> Result<(), rbatis::Error> {
  let rb = rbatis::RBatis::new();
  rb.init(YourDatabaseDriver {}, "database://username:password@host:port/dbname")?;
}
```

## More Information

- [Full Documentation](https://rbatis.github.io/rbatis.io)
- [Changelog](https://github.com/rbatis/rbatis/releases/)
- [rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## Claude Code Integration

This project includes a Claude Code skill for AI-assisted development. For usage, please visit: [https://github.com/rbatis/rbatis-skill](https://github.com/rbatis/rbatis-skill)

## Contact Us

[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

### Donations or Contact

<img style="width: 200px;height: 300px;" width="200" height="300" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> WeChat (Please note 'rbatis' when adding as friend)

<img style="width: 200px;height: 250px;" width="200" height="250" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />


