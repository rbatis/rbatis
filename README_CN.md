# Rbatis

##### 📖 [英文文档](Readme.md) | 📖 中文文档
（机翻中文，如有差异，已英文原版为主）

[Website](https://rbatis.github.io/rbatis.io) | [Showcase](https://github.com/rbatis/rbatis/network/dependents) | [Examples](https://github.com/rbatis/rbatis/tree/master/example)

[![Build Status](https://github.com/rbatis/rbatis/workflows/ci/badge.svg)](https://github.com/zhuxiujia/rbatis/actions)
[![doc.rs](https://docs.rs/rbatis/badge.svg)](https://docs.rs/rbatis/)
[![](https://img.shields.io/crates/d/rbatis)](https://crates.io/crates/rbatis)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![codecov](https://codecov.io/gh/rbatis/rbatis/graph/badge.svg?token=VAVPXSHoff)](https://codecov.io/gh/rbatis/rbatis)
[![GitHub release](https://img.shields.io/github/v/release/rbatis/rbatis)](https://github.com/rbatis/rbatis/releases)
[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

<img style="width: 200px;height: 190px;" width="200" height="190" src="logo.png" />

## 简介

Rbatis 是一个基于编译时代码生成的高性能 Rust ORM 框架。它在开发效率、性能和稳定性之间实现了完美平衡，既是一个 ORM，也是一个动态 SQL 编译器。

## AI 支持
* - [rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## 核心优势

### 1. 高性能
- **🚀 动态SQL直接翻译为原生Rust**：核心优势 - 将动态SQL直接编译为原生Rust代码，达到手写SQL的性能水平
- **零运行时开销**：所有SQL解析和优化都在编译时完成，消除运行时解释成本
- **基于 Tokio 异步模型**：充分利用 Rust 的异步特性，提升并发性能
- **高效的连接池**：内置多种连接池实现，优化数据库连接管理

### 2. 可靠性
- **Rust 安全特性**：利用 Rust 的所有权和借用检查确保内存和线程安全
- **统一参数占位符**：使用 `?` 作为统一占位符，支持所有驱动
- **两种替换模式**：预编译的 `#{arg}` 和直接替换 `${arg}`，满足不同场景需求

### 3. 开发效率
- **强大的 ORM 能力**：数据库表与 Rust 结构体自动映射
- **多种 SQL 构建方式**：
  - **py_sql**：Python 风格的动态 SQL，支持 `if`、`for`、`choose/when/otherwise`、`bind`、`trim` 结构和集合操作（`.sql()`、`.csv()`）
  - **html_sql**：类似 MyBatis 的 XML 模板，具有熟悉的标签结构（`<if>`、`<where>`、`<set>`、`<foreach>`），声明式 SQL 构建，自动处理 SQL 片段而无需 CDATA
  - **原始 SQL**：直接的 SQL 语句
- **CRUD 宏**：一行代码生成通用 CRUD 操作
- **拦截器插件**：[自定义扩展功能](https://rbatis.github.io/rbatis.io/#/v4/?id=plugin-intercept)
- **表同步插件**：[自动创建/更新表结构](https://rbatis.github.io/rbatis.io/#/v4/?id=plugin-table-sync)

### 4. 可扩展性
- **多数据库支持**：MySQL、PostgreSQL、SQLite、MSSQL、MariaDB、TiDB、CockroachDB、Oracle、TDengine 等
- **自定义驱动接口**：实现简单接口即可添加对新数据库的支持
- **多连接池**：FastPool（默认）、Deadpool、MobcPool
- **兼容多种 Web 框架**：与 ntex、actix-web、axum、hyper、rocket、tide、warp、salvo 等无缝集成

## 支持的数据库驱动

| 数据库 (crates.io)                               | GitHub 链接                                                                       |
|----------------------------------------------------|-----------------------------------------------------------------------------------|
| [MySQL](https://crates.io/crates/rbdc-mysql)       | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [PostgreSQL](https://crates.io/crates/rbdc-pg)     | [rbatis/rbdc-pg](https://github.com/rbatis/rbdc/tree/master/rbdc-pg)              |
| [SQLite](https://crates.io/crates/rbdc-sqlite)     | [rbatis/rbdc-sqlite](https://github.com/rbatis/rbdc/tree/master/rbdc-sqlite)      |
| [MSSQL](https://crates.io/crates/rbdc-mssql)       | [rbatis/rbdc-mssql](https://github.com/rbatis/rbdc/tree/master/rbdc-mssql)        |
| [MariaDB](https://crates.io/crates/rbdc-mysql)     | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [TiDB](https://crates.io/crates/rbdc-mysql)        | [rbatis/rbdc-mysql](https://github.com/rbatis/rbdc/tree/master/rbdc-mysql)        |
| [CockroachDB](https://crates.io/crates/rbdc-pg)    | [rbatis/rbdc-pg](https://github.com/rbatis/rbdc/tree/master/rbdc-pg)              |
| [Oracle](https://crates.io/crates/rbdc-oracle)     | [chenpengfan/rbdc-oracle](https://github.com/chenpengfan/rbdc-oracle)             |
| [TDengine](https://crates.io/crates/rbdc-tdengine) | [tdcare/rbdc-tdengine](https://github.com/tdcare/rbdc-tdengine)                   |

## 支持的连接池

| 连接池 (crates.io)                               | GitHub 链接                                                                       |
|-----------------------------------------------------------|-----------------------------------------------------------------------------------|
| [FastPool (默认)](https://crates.io/crates/rbdc-pool-fast) | [rbatis/fast_pool](https://github.com/rbatis/rbatis/tree/master/rbdc-pool-fast) |
| [Deadpool](https://crates.io/crates/rbdc-pool-deadpool)       | [rbatis/rbdc-pool-deadpool](https://github.com/rbatis/rbdc-pool-deadpool)      |
| [MobcPool](https://crates.io/crates/rbdc-pool-mobc)            | [rbatis/rbdc-pool-mobc](https://github.com/rbatis/rbdc-pool-mobc)              |

## 支持的数据类型

| 数据类型                                                               | 支持 |
|-------------------------------------------------------------------------|---------|
| `Option`                                                                | ✓       |
| `Vec`                                                                   | ✓       |
| `HashMap`                                                               | ✓       |
| `i32, i64, f32, f64, bool, String` 及其他 Rust 基础类型           | ✓       |
| `rbatis::rbdc::types::{Bytes, Date, DateTime, Time, Timestamp, Decimal, Json}` | ✓ |
| `rbatis::plugin::page::{Page, PageRequest}`                             | ✓       |
| `rbs::Value`                                                            | ✓       |
| `serde_json::Value` 及其他 serde 类型                               | ✓       |
| 来自 rbdc-mysql, rbdc-pg, rbdc-sqlite, rbdc-mssql 的驱动特定类型 | ✓       |


## 其他库 crates

| crate                                 | GitHub 链接                                     |
|---------------------------------------|-------------------------------------------------|
| [rbdc](https://crates.io/crates/rbdc) | [rbdc](https://github.com/rbatis/rbdc)          |
| [rbs](https://crates.io/crates/rbs)   | [rbs](https://github.com/rbatis/rbs)             |



## Rbatis 工作原理

Rbatis 通过 `rbatis-codegen` crate 使用编译时代码生成，这意味着：

1. **🎯 直接翻译为原生Rust**：动态SQL在编译期间转换为优化的Rust代码，无需任何运行时解释，达到与手写SQL完全相同的性能。

2. **编译过程**：
   - **词法分析**：由 `rbatis-codegen` 中的 `func.rs` 使用 Rust 的 `syn` 和 `quote` crates 处理
   - **语法解析**：由 `rbatis-codegen` 中的 `parser_html` 和 `parser_pysql` 模块执行
   - **抽象语法树**：使用 `rbatis-codegen` 中 `syntax_tree` 包定义的结构构建
   - **中间代码生成**：由 `func.rs` 执行，其中包含所有代码生成函数

3. **构建过程集成**：整个过程在 `cargo build` 阶段作为 Rust 的过程宏编译的一部分运行。生成的代码返回给 Rust 编译器进行 LLVM 编译以生成机器代码。

4. **编译时SQL优化**：与传统ORM在运行时解释动态SQL（导致性能损失）不同，Rbatis在编译时将SQL翻译为原生Rust代码，在保持ORM便利性的同时，提供手写SQL的性能。

## 性能基准测试

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

## 快速开始

### 依赖

```toml
# Cargo.toml
[dependencies]
rbs = { version = "4.6"}
rbatis = { version = "4.6"}
#drivers
rbdc-sqlite = { version = "4.6" }
# rbdc-mysql = { version = "4.6" }
# rbdc-pg = { version = "4.6" }
# rbdc-mssql = { version = "4.6" }

# 其他依赖
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

### 基本用法

```rust
use rbatis::rbdc::datetime::DateTime;
use rbs::value;
use rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub status: Option<i32>,
    pub create_time: Option<DateTime>,
    pub additional_field: Option<String>,
}

// 自动生成 CRUD 方法
crud!(BizActivity{});

#[tokio::main]
async fn main() {
    // 配置日志
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    
    // 初始化 rbatis
    let rb = RBatis::new();
    
    // 连接数据库
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // 或其他数据库
    // rb.init(MysqlDriver{}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(PgDriver{}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    
    // 创建数据
    let activity = BizActivity {
        id: Some("1".into()),
        name: Some("测试活动".into()),
        status: Some(1),
        create_time: Some(DateTime::now()),
        additional_field: Some("额外信息".into()),
    };

    // 插入数据
    let data = BizActivity::insert(&rb, &activity).await;

    // 批量插入
    let data = BizActivity::insert_batch(&rb, &vec![BizActivity {
            id: Some("2".into()),
            name: Some("活动 2".into()),
            status: Some(1),
            create_time: Some(DateTime::now()),
            additional_field: Some("信息 2".into()),
        }, BizActivity {
            id: Some("3".into()),
            name: Some("活动 3".into()),
            status: Some(1),
            create_time: Some(DateTime::now()),
            additional_field: Some("信息 3".into()),
        },
    ], 10).await;

    // 根据 map 条件更新
    let data = BizActivity::update_by_map(&rb, &activity, value!{ "id": "1" }).await;

    // 根据 map 条件查询
    let data = BizActivity::select_by_map(&rb, value!{"id":"2","name":"活动 2"}).await;

    // LIKE 查询
    let data = BizActivity::select_by_map(&rb, value!{"name like ":"%活动%"}).await;

    // 大于查询
    let data = BizActivity::select_by_map(&rb, value!{"id > ":"2"}).await;

    // IN 查询
    let data = BizActivity::select_by_map(&rb, value!{"id": &["1", "2", "3"]}).await;

    // 根据 map 条件删除
    let data = BizActivity::delete_by_map(&rb, value!{"id": &["1", "2", "3"]}).await;
}
```

## 创建自定义数据库驱动

要为 Rbatis 实现自定义数据库驱动：

1. 定义你的驱动项目及依赖：
```toml
[features]
default = ["tls-rustls"]
tls-rustls=["rbdc/tls-rustls"]
tls-native-tls=["rbdc/tls-native-tls"]
[dependencies]
rbs = { version = "4.6"}
rbdc = { version = "4.6", default-features = false, optional = true }
fastdate = { version = "0.3" }
tokio = { version = "1", features = ["full"] }
```

2. 实现所需的 trait：
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

// 然后使用你的驱动：
#[tokio::main]
async fn main() {
  let rb = rbatis::RBatis::new();
  rb.init(YourDatabaseDriver {}, "database://username:password@host:port/dbname").unwrap();
}
```

## 更多信息

- [完整文档](https://rbatis.github.io/rbatis.io)
- [变更日志](https://github.com/rbatis/rbatis/releases/)
- [rbdc-mcp](https://github.com/rbatis/rbdc-mcp)

## 联系我们

[![discussions](https://img.shields.io/github/discussions/rbatis/rbatis)](https://github.com/rbatis/rbatis/discussions)

### 捐赠或联系

<img style="width: 200px;height: 300px;" width="200" height="300" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> 微信（添加好友时请备注 'rbatis'）

<img style="width: 200px;height: 250px;" width="200" height="250" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />
