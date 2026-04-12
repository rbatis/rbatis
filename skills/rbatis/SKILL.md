---
name: rbatis
description: Rust high-performance ORM framework - compile-time dynamic SQL, MyBatis compatible syntax
---

# Rbatis - Rust ORM Framework

> Documentation: https://rbatis.github.io/rbatis.io | GitHub: https://github.com/rbatis/rbatis

## Core Features

- **Compile-time Dynamic SQL**: Dynamic SQL is compiled directly to native Rust code, zero runtime overhead
- **MyBatis Compatible**: Supports `html_sql` (XML template) and `py_sql` (Python-like syntax)
- **Unified Placeholders**: All drivers support `#{arg}`, `${arg}`, `?`
- **Native Async**: Based on Future/Tokio, 100% safe pure Rust (`#![forbid(unsafe_code)]`)

---

## Table of Contents

1. [Dependencies](#1-dependencies)
2. [Initialization](#2-initialization)
3. [crud! Macro](#3-crud-macro)
4. [html_sql](#4-html_sql)
5. [py_sql](#5-py_sql)
6. [Raw SQL](#6-raw-sql)
7. [Transaction](#7-transaction)
8. [Pagination](#8-pagination)
9. [Table Sync](#9-table-sync)
10. [Interceptor](#10-interceptor)
11. [Built-in Macros](#11-built-in-macros)
12. [Driver Design](#12-driver-design)

---

## 1. Dependencies

```toml
# Cargo.toml
[dependencies]
rbatis = { version = "4.8"}
rbs = { version = "4"}
rbdc-sqlite = { version = "4" }
# rbdc-mysql = { version = "4" }
# rbdc-pg = { version = "4" }
# rbdc-mssql = { version = "4" }

serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

**With native-tls:**
```toml
rbdc-sqlite = { version = "4", default-features = false, features = ["tls-native-tls"] }
# rbdc-mysql = { version = "4", default-features = false, features = ["tls-native-tls"] }
# rbdc-pg = { version = "4", default-features = false, features = ["tls-native-tls"] }
```

---

## 2. Initialization

```rust
use rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;

#[tokio::main]
async fn main() -> rbatis::Result<()> {
    fast_log::init(fast_log::Config::new().console())?;

    let rb = RBatis::new();

    // init() only sets the driver
    // rb.init(SqliteDriver {}, "sqlite://target/sqlite.db")?;

    // link() sets driver and connects to database
    rb.link(SqliteDriver {}, "sqlite://target/sqlite.db").await?;

    Ok(())
}
```

**Database connection strings:**
```rust
// SQLite
rb.link(SqliteDriver {}, "sqlite://target/sqlite.db").await?;

// MySQL
rb.link(MysqlDriver {}, "mysql://root:123456@localhost:3306/test").await?;

// PostgreSQL
rb.link(PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").await?;

// MSSQL
rb.link(MssqlDriver {}, "jdbc:sqlserver://localhost:1433;User=SA;Password=TestPass!123456;Database=test").await?;
```

---

## 3. crud! Macro

The `crud!` macro generates: `insert`, `insert_batch`, `select_by_map`, `update_by_map`, `delete_by_map`

### Define Table Structure

```rust
use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::DateTime;
use rbs::value;

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

// Table name auto-inferred as snake_case ("biz_activity")
crud!(BizActivity {});

// Custom table name
crud!(BizActivity {}, "biz_activity");
```

### insert

```rust
let table = BizActivity {
    id: Some("2".into()),
    name: Some("2".into()),
    // ... other fields
};

// Single insert
BizActivity::insert(&rb, &table).await?;

// Batch insert
let tables = [table.clone(), { /* ... */ }];
BizActivity::insert_batch(&rb, &tables, 10).await?;  // batch_size=10
```

### update

```rust
// Update by condition, fields in condition are used as WHERE
BizActivity::update_by_map(&rb, &table, value!{"id": &table.id}).await?;
BizActivity::update_by_map(&rb, &table, value!{"id": "1"}).await?;
```

### select

```rust
// Equality condition
BizActivity::select_by_map(&rb, value!{"id":"1"}).await?;

// Multiple conditions
BizActivity::select_by_map(&rb, value!{"id":"1","name":"test"}).await?;

// IN query
BizActivity::select_by_map(&rb, value!{"id": &["1", "2", "3"]}).await?;

// LIKE query (key ends with " like ")
BizActivity::select_by_map(&rb, value!{"name like ": "%test%"}).await?;

// Comparison operators
BizActivity::select_by_map(&rb, value!{"id > ": "2"}).await?;
```

### delete

```rust
BizActivity::delete_by_map(&rb, value!{"id": "1"}).await?;
BizActivity::delete_by_map(&rb, value!{"id": &["1", "2", "3"]}).await?;
```

---

## 4. html_sql

`html_sql` is a MyBatis-like XML template syntax. SQL is compiled to Rust code at compile time.

### Basic Syntax

**Inline HTML:**
```rust
use rbatis::html_sql;
use rbatis::executor::Executor;

#[html_sql(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<select id="select_by_condition">
    `select * from biz_activity`
    <where>
        <if test="a">
            ` and name like #{name}`
        </if>
        <if test="name != ''">
            ` and name like #{name}`
        </if>
        <if test="dt >= '2009-12-12 00:00:00'">
            ` and create_time < #{dt}`
        </if>
        <choose>
            <when test="true">
                ` and id != '-1'`
            </when>
            <otherwise>and id != -2</otherwise>
        </choose>
        ` and `
        <trim prefixOverrides=" and">
            ` and name != '' `
        </trim>
    </where>
</select>"#)]
async fn select_by_condition(
    rb: &dyn Executor,
    name: &str,
    dt: &DateTime,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}
```

**Load from file:**
```rust
#[html_sql("example/example.html")]
impl BizActivity {
    pub async fn select_by_condition(
        rb: &dyn Executor,
        name: &str,
        dt: &DateTime,
    ) -> rbatis::Result<Vec<BizActivity>> {
        impled!()
    }
}
```

**Using htmlsql! macro:**
```rust
htmlsql!(select_by_condition(
    rb: &dyn Executor,
    name: &str,
    dt: &DateTime
) -> rbatis::Result<Vec<BizActivity>> => "example.html");
```

### HTML Template Structure

```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <!-- Reusable SQL fragment -->
    <sql id="a">` and id != '' `</sql>

    <select id="select_by_condition">
        `select * from biz_activity`
        <where>
            <!-- if: conditional -->
            <if test="name != ''">
                ` and name like #{name}`
            </if>

            <!-- choose/when/otherwise: switch/case -->
            <choose>
                <when test="true">
                    ` and id != '-1'`
                </when>
                <otherwise>and id != -2</otherwise>
            </choose>

            <!-- trim: remove/add prefix/suffix -->
            <trim prefixOverrides=" and">
                ` and name != '' `
            </trim>
        </where>
    </select>

    <!-- foreach: iterate collection -->
    <insert id="insert">
        `insert into biz_activity`
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            <if test="key == 'id'">
                <continue></continue>
            </if>
            ${key}
        </foreach>
    </insert>

    <!-- set: UPDATE SET -->
    <update id="update_by_id">
        ` update biz_activity `
        <set collection="arg"></set>
        ` where id = #{id} `
    </update>
</mapper>
```

### html_sql Common Usage

**String concatenation (for LIKE patterns):**
```html
` and title like #{'%'+title+'%'}`
` and name like #{'%'+name+'%'}`
```

**Date range query:**
```html
` and create_date >= #{start_date}`
` and create_date <= #{end_date}`
```

**No escaping needed in HTML:** Comparison operators like `<=` and `>=` can be written directly without XML escaping.

### html_sql Syntax Tree

| Syntax | Generated Rust Code |
|--------|---------------------|
| `` <trim prefixOverrides=" and">` and name != '' `</trim> `` | `sql.trim(" and")` |
| `` <if test="key == 'id'">`select * from table`</if> `` | `if key == "id"{sql.push_str("select * from table");}` |
| `` <foreach collection="arg" index="key" item="item" open="(" close=")" separator=","/> `` | `for (key,item) in arg{}` |
| `` <continue></continue> `` | `continue;` |
| `` <break></break> `` | `break;` |
| `` <set> `` | `sql.trim("set ").push_str(" set ");` |
| `` <set collection="arg"> `` | `sql.trim("set ").push_str(" set name=?,age=? ");` |
| `` <choose> `` | `match {}` |
| `` <when test="true"> `` | `match true{ true=>{} _ => {} }` |
| `` <otherwise> `` | `match { _ =>{} }` |
| `` <where> `` | `sql.push_str("WHERE").trim("WHERE");` |
| `` <bind name="a" value="1+1"></bind> `` | `let a = rbs::Value::I32(1 + 1);` |
| `` `select * from table` `` | `sql.push_str("select * from table");` |
| `` `#{name}` `` | `sql.push_str("?");args.push(rbs::Value::String(name));` |
| `` `${name}` `` | `sql.push_str(&format!("{}",name));` |
| `` `${1 + 1}` `` | `sql.push_str(&format!("{}", 1 + 1));` |
| `` `#{1 + 1}` `` | `sql.push_str("?");args.push(rbs::Value::from(1+1));` |
| `` `${name + '_tag'}` `` | `sql.push_str(&format!("{}",name + "_tag"));` |
| `` `#{name + '_tag'}` `` | `sql.push_str("?");args.push(rbs::Value::from(format!("{}",name + "_tag")));` |
| `` `${age + 1}` `` | `sql.push_str(&format!("{}", age + 1));` |
| `` `#{age + 1}` `` | `sql.push_str("?");args.push(rbs::Value::from(age+1));` |
| `` `${true & true}` `` | `sql.push_str(&format!("{}", true & true));` |
| `` `#{true & true}` `` | `sql.push_str("?");args.push(rbs::Value::from(true & true));` |
| `` `${2 > 1}` `` | `sql.push_str(&format!("{}",2 > 1));` |
| `` `${2 / 1}` `` | `sql.push_str(&format!("{}", 2 / 1));` |
| `` `${2 == 1}` `` | `sql.push_str(&format!("{}", 2 == 1));` |
| `` `${!false}` `` | `sql.push_str(&format!("{}", !false));` |
| `` `${2 % 1}` `` | `sql.push_str(&format!("{}", 2 % 1));` |
| `` `${2 - 1}` `` | `sql.push_str(&format!("{}", 2 - 1));` |

### include - SQL Fragment Reuse

```html
<!-- Define fragment -->
<sql id="a">` and id != '' `</sql>

<!-- Reference in same file -->
<include refid="a"></include>

<!-- Reference from external file -->
<include refid="file://../rbatis/example/example.html?refid=a"></include>
```

---

## 5. py_sql

`py_sql` is a Python-like dynamic SQL syntax.

### Basic Syntax

```rust
use rbatis::py_sql;
use rbatis::executor::Executor;

#[py_sql(
    "`select * from user where delete_flag = 0`
    if name != '':
        ` and name=#{name}`"
)]
async fn py_select(rb: &dyn Executor, name: &str) -> Result<Vec<User>, Error> {
    impled!()
}
```

**Using pysql! macro:**
```rust
pysql!(user_delete_by_name(rb: &dyn Executor, name: &str) -> Result<ExecResult, Error> =>
    "`delete from user where delete_flag = 0`
    if name != '':
        ` and name=#{name}`" );

// In impl block
impl User {
    pysql!(user_delete_by_name(rb: &dyn Executor, name: &str) -> Result<ExecResult, Error> =>
        "`delete from user where delete_flag = 0`
        if name != '':
            ` and name=#{name}`" );
}
```

### py_sql Common Usage

**String concatenation (for LIKE patterns):**
```python
` and title like #{'%'+title+'%'}`
` and name like #{'%'+name+'%'}`
```

**Date range query:**
```python
` and create_date >= #{start_date}`
` and create_date <= #{end_date}`
```

**Collection to SQL list (.sql() method):**
```python
`select * from activity where delete_flag = 0`
if !ids.is_empty():
    ` and id in `
    ${ids.sql()}
```
Result: `select * from activity where delete_flag = 0 and id in (1, 2, 3)`

### py_sql Syntax Tree

| Syntax | Generated Rust Code |
|--------|---------------------|
| `` trim 'AND ': `` | `sql.trim_end_matches("AND ").trim_start_matches("AND ")` |
| `` trim start='AND ': `` | `sql.trim_start_matches("AND ")` |
| `` trim end='AND ': `` | `sql.trim_end_matches("AND ")` |
| `` if arg!=1: `` | `if arg !=1 {}` |
| `` if true:` ``<br/>`` `select * from table` `` | `if true { sql.push_str("select * from table");}` |
| `` for key,item in arg: `` | `for (key,item) in arg{ }` |
| `` for key,item in arg:` ``<br/>`` `and name = ${name}` `` | `for (key,item) in arg{ sql.push_str(&format!("and name = {}",name)); }` |
| `` for key,item in arg:` ``<br/>`` `continue:` `` | `for (key,item) in arg{ continue; }` |
| `` set : `` | `sql.push_str("SET")` |
| `` set collection='ids': `` | `sql.trim("set ").push_str(" set name=?,age=? ");` |
| `` choose : `` | `match {}` |
| `` when : `` | `match true{ true=>{} _ => {} }` |
| `` otherwise : `` | `match { _ =>{} }` |
| `` _: `` | `match { _ =>{} }(v1.8.54+)` |
| `` where : `` | `sql.push_str("WHERE").trim("WHERE")` |
| `` bind a=1+1: `` | `let a = rbs::Value::I32(1 + 1);` |
| `` let a=1+1: `` | `let a = rbs::Value::I32(1 + 1);` (v1.8.54+) |
| `` `#{name}` `` | `sql.push_str("?");args.push(rbs::Value::String(name));` |
| `` `${name}` `` | `sql.push_str(&format!("{}",name));` |
| `` `${1 + 1}` `` | `sql.push_str(&format!("{}", 1 + 1));` |
| `` `#{1 + 1}` `` | `sql.push_str("?");args.push(rbs::Value::from(1+1));` |
| `` `${name + '_tag'}` `` | `sql.push_str(&format!("{}",name.to_string() + "_tag"));` |
| `` `#{name + '_tag'}` `` | `sql.push_str("?");args.push(rbs::Value::from(format!("{}",name + "_tag")));` |
| `` `${age + 1}` `` | `sql.push_str(&format!("{}", age + 1));` |
| `` `#{age + 1}` `` | `sql.push_str("?");args.push(rbs::Value::from(age+1));` |
| `` `${true & true}` `` | `sql.push_str(&format!("{}", true & true));` |
| `` `#{true & true}` `` | `sql.push_str("?");args.push(rbs::Value::from(true & true));` |
| `` `${2 > 1}` `` | `sql.push_str(&format!("{}",2 > 1));` |
| `` `${2 / 1}` `` | `sql.push_str(&format!("{}", 2 / 1));` |
| `` `${2 == 1}` `` | `sql.push_str(&format!("{}", 2 == 1));` |
| `` `${!false}` `` | `sql.push_str(&format!("{}", !false));` |
| `` `${2 % 1}` `` | `sql.push_str(&format!("{}", 2 % 1));` |
| `` `${2 - 1}` `` | `sql.push_str(&format!("{}", 2 - 1));` |

---

## 6. Raw SQL

Use native SQL directly, driver handles placeholder `?` conversion automatically.

```rust
use rbs::value;

// Execute SQL (INSERT/UPDATE/DELETE)
let result = rb
    .exec("update activity set status = 0 where id > 0", vec![])
    .await?;

// Query and decode result
let table: Option<BizActivity> = rb
    .exec_decode("select * from biz_activity limit ?", vec![value!(1)])
    .await?;
```

---

## 7. Transaction

### Basic Transaction

```rust
let tx = rb.acquire_begin().await?;
BizActivity::insert(&tx, &table).await?;
tx.commit().await?;
tx.rollback().await?;
```

### Auto Rollback (defer_async)

```rust
let tx = rb.acquire_begin().await?;

// defer_async: rollback automatically if transaction is dropped without commit
let tx = tx.defer_async(|tx| async move {
    if tx.done() {
        log::info!("transaction [{}] complete", tx.tx_id());
    } else {
        tx.rollback().await.unwrap();
        log::info!("transaction [{}] rollback", tx.tx_id());
    }
});

BizActivity::insert(&tx, &table).await?;
tx.commit().await?;  // after commit, tx.done() == true, no rollback
```

---

## 8. Pagination

### Auto Pagination

When return type is `Page<T>`, `PageIntercept` handles pagination automatically.

**Using impl block (recommended):**
```rust
#[rbatis::html_sql("example/example.html")]
impl Activity {
    // PageIntercept automatically adds LIMIT/OFFSET
    pub async fn select_by_page(
        rb: &dyn rbatis::Executor,
        page_req: &rbatis::PageRequest,
        name: &str,
    ) -> rbatis::Result<rbatis::Page<Activity>> {
        impled!()
    }
}

// Usage
let page = Activity::select_by_page(&rb, &PageRequest::new(1, 10), "test").await?;
```

**Using htmlsql_select_page! macro:**
```html
<!-- example.html -->
<select id="select_page_data">
    `select `
    <if test="do_count == true">
        ` count(1) from table`
    </if>
    <if test="do_count == false">
        ` * from table limit ${page_no},${page_size}`
    </if>
</select>
```

```rust
use rbatis::htmlsql_select_page;
use rbatis::rbdc::datetime::DateTime;
use rbatis::PageRequest;

htmlsql_select_page!(select_page_data(name: &str, dt: &DateTime) -> BizActivity => "example/example.html");

// Usage
let page = select_page_data(
    &rb,
    &PageRequest::new(1, 10),
    "test",
    &DateTime::now(),
).await?;
```

### Page Type

```rust
use rbatis::Page;
use rbatis::PageRequest;

let page_req = PageRequest::new(1, 10);

// Query with pagination
let page: Page<Activity> = Activity::select_by_page(&rb, &page_req, "test").await?;

// Page fields
page.total;      // total records
page.pages;      // total pages
page.page_no;    // current page
page.page_size;  // page size
page.records;    // data Vec<T>
```

---

## 9. Table Sync

Auto-sync Rust struct to database table (creates table/add columns only, never modifies existing columns).

```rust
use rbatis::table_sync;
use rbatis::RBatis;

#[tokio::main]
pub async fn main() {
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();

    // Select mapper for your database
    let mapper = &table_sync::SqliteTableMapper {};
    // let mapper = &table_sync::PGTableMapper{};
    // let mapper = &table_sync::MysqlTableMapper{};
    // let mapper = &table_sync::MssqlTableMapper{};

    // Method 1: Using value! macro to define column types
    let table = value! {
        "id": "INTEGER",
        "name": "TEXT",
        "remark": "TEXT",
        "create_time": "TEXT",
        "version": "TEXT",
        "delete_flag": "INT8"
    };
    RBatis::sync(&rb.acquire().await.unwrap(), mapper, &table, "rb_user").await?;

    // Method 2: Using struct directly
    RBatis::sync(
        &rb.acquire().await.unwrap(),
        mapper,
        &BizActivity {
            id: 0,
            name: Some("".to_string()),
            // ... other fields
            create_time: Some(DateTime::now()),
            version: Some(1),
            delete_flag: Some(1),
        },
        "biz_activity",  // table name
    ).await?;
}
```

---

## 10. Interceptor

### Implement Intercept Trait

```rust
use rbatis::async_trait;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::ExecResult;
use rbatis::{Action, Error};
use rbs::Value;
use std::sync::Arc;

#[derive(Debug)]
pub struct MyInterceptor {}

#[async_trait]
impl Intercept for MyInterceptor {
    /// before: called before SQL execution
    /// Return Action::Next to continue, Action::Stop to stop
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        Ok(Action::Next)  // continue
    }

    /// after: called after SQL execution
    async fn after(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        Ok(Action::Next)
    }
}

// Add to RBatis
fn main() {
    let mut rb = RBatis::new();
    rb.intercepts.push(Arc::new(MyInterceptor {}) as Arc<dyn Intercept>);
}
```

### Built-in Interceptors

| Interceptor | Description |
|-------------|-------------|
| `PageIntercept` | Auto pagination |
| `LogInterceptor` | SQL logging (enabled by default) |

---

## 11. Built-in Macros

### make_table! - Simplified Table Construction

```rust
use rbatis::make_table;

#[test]
fn test_make_table() {
    let table = rbatis::make_table!(BizActivity{
        id: "1".to_string(),
    });
    println!("{:#?}", table);
}
```

### make_table_field_map! - Get HashMap

```rust
#[test]
fn test_table_field_map() {
    let table = rbatis::make_table!(BizActivity{
        id: "1".to_string(),
        name: "a".to_string()
    });
    let table_vec = vec![table];
    let map = rbatis::make_table_field_map!(&table_vec, name);
    println!("{:#?}", map);
}
```

### make_table_field_vec! - Get Vec

```rust
#[test]
fn test_table_field_vec() {
    let table = rbatis::make_table!(BizActivity{
        id: "1".to_string(),
        name: "a".to_string()
    });
    let table_vec = vec![table];
    let names = rbatis::make_table_field_vec!(&table_vec, name);
    println!("{:#?}", names);
}
```

---

## 12. Driver Design

Implement `rbdc::db::*` traits to create custom database drivers.

### Required Traits

```rust
use rbdc::db::{Driver, MetaData, Row, Connection, ConnectOptions, Placeholder};
```

### Implementation Steps

```rust
// step1: define structs
#[derive(Debug, Clone)]
struct MockDriver {}
#[derive(Clone, Debug)]
struct MockRowMetaData {}
#[derive(Clone, Debug)]
struct MockRow {}
#[derive(Clone, Debug)]
struct MockConnection {}
#[derive(Clone, Debug)]
struct MockConnectOptions {}

// step2: implement traits
impl Driver for MockDriver {
    fn name(&self) -> &str { "MockDriver" }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<'a, Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(MockConnectOptions {})
    }
}

impl MetaData for MockRowMetaData {
    fn column_len(&self) -> usize { todo!() }
    fn column_name(&self, i: usize) -> String { todo!() }
    fn column_type(&self, i: usize) -> String { todo!() }
}

impl Row for MockRow {
    fn meta_data(&self) -> Box<dyn MetaData> { todo!() }
    fn get(&mut self, i: usize) -> Result<Value, Error> { todo!() }
}

impl Connection for MockConnection {
    fn exec_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> { todo!() }
    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> { todo!() }
    fn close(&mut self) -> BoxFuture<Result<(), Error>> { todo!() }
    fn ping(&mut self) -> BoxFuture<Result<(), Error>> { todo!() }
}

impl ConnectOptions for MockConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> { todo!() }
    fn set_uri(&mut self, uri: &str) -> Result<(), Error> { todo!() }
}

impl Placeholder for MockDriver {
    fn exchange(&self, sql: &str) -> String {
        // If database doesn't support ? placeholder, convert to database-specific format
        // e.g., MSSQL: sql.replacen("?", "@P1", 1)
        sql.to_string()
    }
}

// step3: use the driver
#[tokio::main]
async fn main() {
    let mut rb = RBatis::new();
    rb.init(MockDriver {}, "xxx://xxx.db").unwrap();
    rb.acquire().await.expect("connect database fail");
}
```

---

## Appendix: rbs::Value Types

`rbs` is rbatis's serialization framework, similar to JSON Value.

```rust
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Binary(Vec<u8>),
    Array(Vec<Self>),
    Map(ValueMap),
    Ext(&'static str, Box<Self>),
}

// Build value
let v = rbs::to_value!{ "key": "value" };

// Decode
let v: i32 = rbs::from_value(Value::I32(1)).unwrap();
```

---

## Appendix: Placeholder Reference

| Syntax | Type | Description |
|--------|------|-------------|
| `#{arg}` | Prepared placeholder | Parameter bound as `?`, safe from injection |
| `${arg}` | Direct string replacement | Concatenated directly into SQL |
| `?` | Native placeholder | Driver auto-converts to database format |

**Examples:**
```sql
-- #{name} generates prepared statement
` where name = #{name}`  ->  sql.push_str("?"); args.push(name)

-- ${name} direct replacement
` where name = ${name}`  ->  sql.push_str(&format!("{}", name))
```

---

