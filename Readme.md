[WebSite](https://rbatis.github.io/rbatis.io/#/en/) | [简体中文](https://rbatis.github.io/rbatis.io/)

[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)
[![doc.rs](https://docs.rs/rbatis/badge.svg)](https://docs.rs/rbatis/)
[![](https://img.shields.io/crates/d/rbatis)](https://crates.io/crates/rbatis)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/) </br>
[![dependency status](https://deps.rs/crate/rbatis/1.8.71/status.svg)](https://deps.rs/crate/rbatis/1.8.71)
[![GitHub release](https://img.shields.io/github/v/release/rbatis/rbatis)](https://github.com/rbatis/rbatis/releases)
[![Gitter](https://badges.gitter.im/rbatis_orm/community.svg)](https://gitter.im/rbatis_orm/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

<img style="width: 200px;height: 200px;" width="200" height="200" src="logo.png" />

#### A highly Performant,Safe,Dynamic SQL(Compile time) ORM framework written in Rust, inspired by Mybatis and MybatisPlus.

### [Who Is Using Rbatis](WhoUse.md)

##### Why not diesel or not sqlx ?

| Framework    | Async/.await | Learning curve | Dynamic SQL/py/Wrapper/built-in CRUD | Logical delete plugin| Pagination plugin
| ------ | ------ |------ |------ |------ |------ |
| rbatis | √     | easy   |   √     |    √     |   √     |  
| sqlx   | √     | hard (depends on macros and env. variables) |   x     |   x     |   x     |  
| diesel | x     | hard (use FFI, unsafe) |   x     |  x     |  x     |  


##### Performance comparison with Golang (in a docker environment)

| Framework     | Mysql（docker） | SQL statement（10k） | ns/operation（lower is better） | Qps(higher is better) |Memory usage(lower is better） |
|  ------ | ------ |------ |------ |------ |------ |
| Rust-rbatis/tokio  |  1 CPU, 1G memory    | select count(1) from table;    | 965649 ns/op   |  1035 Qps/s  |  2.1MB   |      
| Go-GoMybatis/http   |  1 CPU, 1G memory   | select count(1) from table;   | 1184503 ns/op  |  844  Qps/s   |  28.4MB  |     

* No Runtimes，No Garbage Collection
* Zero cost [Dynamic SQL](dyn_sql.md), implemented using (proc-macro,compile-time,Cow(Reduce unnecessary cloning))
  techniques。 don't need ONGL engine(mybatis)
* Free deserialization, Auto Deserialize to any struct(Option,Map,Vec...)
* High performance, Based on Future, with async_std/tokio, single threaded benchmark can easily achieve 200,000 QPS
* logical deletes, pagination, py-like SQL and basic Mybatis functionalities.
* Supports logging, customizable logging based on `log` crate
* 100% Safe Rust with `#![forbid(unsafe_code)]` enabled
* [rbatis/example (import into Clion!)](example/src)
* [abs_admin project](https://github.com/rbatis/abs_admin)  an complete background user management system(
  Vue.js+rbatis+actix-web)

### Supported data structures

| data structure    | is supported |
| ------ | ------ |
| Option                   | √     | 
| Vec                      | √     |  
| HashMap                      | √     |
| i32,i64,f32,f64,bool,String...more rust type   | √     |  
| rbatis::Bytes                   | √     |  
| rbatis::DateNative              | √     |  
| rbatis::DateUtc                  | √     |  
| rbatis::DateTimeNative          | √     |  
| rbatis::DateTimeUtc             | √     |  
| rbatis::Decimal                 | √     |  
| rbatis::Json<T>                 | √     |  
| rbatis::TimeNative              | √     |  
| rbatis::TimeUtc                 | √     |  
| rbatis::Timestamp               | √     |  
| rbatis::TimestampZ              | √     |  
| rbatis::Uuid                    | √     |  
| rbatis::plugin::page::{Page<T>, PageRequest} | √     |
| rbson::Bson*                      | √     |
| serde_json::*        | √     |
| any serde type         | √     |

### Supported database √supported .WIP

| database    | is supported |
| ------ | ------ |
| Mysql            | √     |   
| Postgres         | √     |  
| Sqlite           | √     |  
| Mssql/Sqlserver            | √(50%)     |  
| MariaDB(Mysql)             | √     |
| TiDB(Mysql)             | √     |
| CockroachDB(Postgres)      | √     |

### Supported OS/Platforms

| platform   | is supported |
| ------ | ------ |
| Linux                   | √     | 
| Apple/MacOS             | √     |  
| Windows               | √     |

### Supported Web Frameworks

* [actix-web](example/src/actix_web/main.rs)
* [axum](example/src/axum/main.rs)
* [hyper](example/src/hyper/main.rs)
* [ntex](example/src/ntex/main.rs)
* [rocket](example/src/rocket/main.rs)
* [tide](example/src/tide/main.rs)
* [warp](example/src/warp/main.rs)
* [salvo](example/src/salvo/main.rs)


##### Quick example: QueryWrapper and common usages (see example/crud_test.rs for details)

* Cargo.toml
``` rust
# add this library,and cargo install

# bson (required)
serde = { version = "1", features = ["derive"] }
rbson = "2.0"

# logging lib(required)
log = "0.4"
fast_log="1.3"

# rbatis (required) default is all-database+runtime-async-std-rustls
rbatis =  { version = "3.0" } 
# also if you use actix-web+mysql
# rbatis = { version = "3.0", default-features = false, features = ["mysql","runtime-async-std-rustls"] }
```

```rust
//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;

use rbatis::crud::CRUD;

/// may also write `CRUDTable` as `impl CRUDTable for BizActivity{}`
/// #[crud_table]
/// #[crud_table(table_name:biz_activity)]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag")]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
#[crud_table]
#[derive(Clone, Debug)]
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
  pub create_time: Option<rbatis::DateTimeNative>,
  pub version: Option<i32>,
  pub delete_flag: Option<i32>,
}

// this macro will create impl BizActivity{ pub fn id()->&str ..... }
impl_field_name_method!(BizActivity{id,name});

/// (optional) manually implement instead of using `derive(CRUDTable)`. This allows manually rewriting `table_name()` function and supports  code completion in IDE.
/// (option) but this struct require  #[derive(Serialize,Deserialize)]
// use rbatis::crud::CRUDTable;
//impl CRUDTable for BizActivity { 
//    fn table_name()->String{
//        "biz_activity".to_string()
//    }
//    fn table_columns()->String{
//        "id,name,delete_flag".to_string()
//    }
//}
#[tokio::main]
async fn main() {
  /// enable log crate to show sql logs
  fast_log::init(fast_log::config::Config::new().console());
  /// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
  let rb = Rbatis::new();
  /// connect to database  
  rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
  /// customize connection pool parameters (optional)
// let mut opt =PoolOptions::new();
// opt.max_size=100;
// rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
  /// newly constructed wrapper sql logic
  let wrapper = rb.new_wrapper()
          .eq("id", 1)                    //sql:  id = 1
          .and()                          //sql:  and 
          .ne(BizActivity::id(), 1)       //sql:  id <> 1
          .in_array("id", &[1, 2, 3])     //sql:  id in (1,2,3)
          .not_in("id", &[1, 2, 3])       //sql:  id not in (1,2,3)
          .like("name", 1)                //sql:  name like 1
          .or()                           //sql:  or
          .not_like(BizActivity::name(), "asdf")       //sql:  name not like 'asdf'
          .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")//sql:  create_time between '2020-01-01 00:00:00' and '2020-01-01 00:00:00'
          .group_by(&["id"])              //sql:  group by id
          .order_by(true, &["id", "name"])//sql:  group by id,name
          ;

  let activity = BizActivity {
    id: Some("12312".to_string()),
    name: None,
    pc_link: None,
    h5_link: None,
    pc_banner_img: None,
    h5_banner_img: None,
    sort: None,
    status: None,
    remark: None,
    create_time: Some(rbatis::DateTimeNative::now()),
    version: Some(1),
    delete_flag: Some(1),
  };
  /// saving
  rb.save(&activity, &[]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

  /// batch saving
  rb.save_batch(&vec![activity], &[]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

  /// fetch allow None or one result. column you can use BizActivity::id() or "id"
  let result: Option<BizActivity> = rb.fetch_by_column(BizActivity::id(), "1").await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

  /// query all
  let result: Vec<BizActivity> = rb.list().await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1

  ///query by id vec
  let result: Vec<BizActivity> = rb.list_by_column("id", &["1"]).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?) 

  ///query by wrapper
  let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper(rb.new_wrapper().eq("id", "1")).await;
//Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

  ///delete
  rb.remove_by_column::<BizActivity, _>("id", &"1").await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

  ///delete batch
  rb.remove_batch_by_column::<BizActivity, _>("id", &["1", "2"]).await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  ) 

  ///update
  let mut activity = activity.clone();
  let r = rb.update_by_column("id", &activity).await;
//Exec   ==> update biz_activity set  status = ?, create_time = ?, version = ?, delete_flag = ?  where id = ?
  rb.update_by_wrapper(&activity, rb.new_wrapper().eq("id", "12312"), &[Skip::Value(&serde_json::Value::Null), Skip::Column("id")]).await;
//Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ? 
}

///...more usage,see crud.rs
```

#### macros (new addition)

* Important update (pysql removes runtime, directly compiles to static rust code)    This means that the performance of
  SQL generated using py_sql,html_sql is roughly similar to that of handwritten code.

> Because of the compile time, the annotations need to declare the database type to be used

```rust
    #[py_sql("select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    async fn py_sql_tx(rb: &Rbatis, tx_id: &String, name: &str) -> Vec<BizActivity> { impled!() }
```

* Added html_sql support, a form of organization similar to MyBatis, to facilitate migration of Java systems to Rust(
  Note that it is also compiled as Rust code at build time and performs close to handwritten code)  this is very faster

> Because of the compile time, the annotations need to declare the database type to be used

```html
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://github.com/rbatis/rbatis_sql/raw/main/mybatis-3-mapper.dtd">
<mapper>
    <select id="select_by_condition">
        select * from biz_activity where
        <if test="name != ''">
            name like #{name}
        </if>
    </select>
</mapper>
```

```rust
    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[html_sql("example/example.html")]
    async fn select_by_condition(rb: &mut RbatisExecutor<'_,'_>, page_req: &PageRequest, name: &str) -> Page<BizActivity> { impled!() }
```

```rust
use once_cell::sync::Lazy;
pub static RB:Lazy<Rbatis> = Lazy::new(||Rbatis::new());

/// Macro generates execution logic based on method definition, similar to @select dynamic SQL of Java/Mybatis
/// RB is the name referenced locally by Rbatis, for example DAO ::RB, com:: XXX ::RB... Can be
/// The second parameter is the standard driver SQL. Note that the corresponding database parameter mysql is? , pg is $1...
/// macro auto edit method to  'pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}'
///
#[sql("select * from biz_activity where id = ?")]
pub async fn select(rb: &Rbatis,name: &str) -> BizActivity {}
//or： pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}

#[tokio::test]
pub async fn test_macro() {
    fast_log::init(fast_log::config::Config::new().console());
    RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
    let a = select(&RB,"1").await.unwrap();
    println!("{:?}", a);
}
```

### Progress - in sequential order

| function    | is supported |
| ------ | ------ |
| CRUD, with built-in CRUD template (built-in CRUD supports logical deletes)                  | √     |
| LogSystem (logging component)                                          | √     | 
| Tx(task/Nested transactions)                                | √     |   
| Py(using py-like  statement in SQL)                         | √     | 
| async/await support                                             | √     | 
| PagePlugin(Pagincation)                                         | √     |
| LogicDelPlugin                                 | √    |
| Html(xml)   Compile time dynamic SQL)                         | √   | 
| DataBase Table ConvertPage(Web UI,Coming soon)                          | x     | 

* Conlusion: Assuming zero time consumed on IO, single threaded benchmark achieves 200K QPS or QPS, which is a few times
  more performant than GC languages like Go or Java.

### FAQ

* Postgres Types Define Please see Doc<br/>

> [中文](https://rbatis.github.io/rbatis.io/#/?id=%e6%95%b0%e6%8d%ae%e5%ba%93%e5%88%97%e6%a0%bc%e5%bc%8f%e5%8c%96%e5%ae%8f)

> [English Doc](https://rbatis.github.io/rbatis.io/#/en/?id=database-column-formatting-macro)

* Support for DateTime and BigDecimal? <br/>
  Currently supports chrono::rbatis::DateTimeNative和bigdecimal::BigDecimal
* Supports for `async/.await` <br/>
  Currently supports both `async_std` and `tokio`
* Stmt in postgres uses ```$1, $2``` instead of ```?``` in Mysql, does this require some special treatment? No, because rbatis uses
  ```#{}``` to describe parametric variabls, you only need to write the correct parameter names and do not need to match it
  with the symbols used by the database.
* Supports for Oracle database driver? <br/>
  No, moving away from IOE is recommended.
* Which crate should be depended on if only the driver is needed? <br/>
  rbatis-core， Cargo.toml add rbatis-core = "*"
* How to select `async/.await` runtime? <br/>
  see https://rbatis.github.io/rbatis.io/#/en/
* column "id" is of type uuid but expression is of type text'? <br/>
  see https://rbatis.github.io/rbatis.io/#/en/?id=database-column-formatting-macro
* How to use '::uuid','::timestamp' on PostgreSQL? <br/>
  see https://rbatis.github.io/rbatis.io/#/en/?id=database-column-formatting-macro

# [Changelog](https://github.com/rbatis/rbatis/releases/)

# [Roadmap](roadmap.md)

# Contact/donation, or click on star [rbatis](https://github.com/rbatis/rbatis)

* [![Gitter](https://badges.gitter.im/rbatis_orm/community.svg)](https://gitter.im/rbatis_orm/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

# 联系方式/捐赠,或 [rbatis](https://github.com/rbatis/rbatis) 点star

> 捐赠

<img style="width: 400px;height: 600px;" width="400" height="600" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> 联系方式(添加好友请备注'rbatis') 微信群：先加微信，然后拉进群

<img style="width: 400px;height: 500px;" width="400" height="500" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />


