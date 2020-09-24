
#### A ORM formwork Rustlang-based,dynamic sql, no Runtime,No Garbage Collector, low Memory use,High Performance orm Framework. support async_std,tokio
#### This crate uses #![forbid(unsafe_code)] to ensure everything is implemented in 100% Safe Rust.
#### rbatis 是一个无GC无虚拟机无运行时Runtime直接编译为机器码,并发安全的  数据库 ORM框架，并且所有数据传值均使用json（serde_json）使用百分之百的安全代码实现

[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)

![Image text](logo.png)


##### way not diesel,way not sqlx ? 为什么不选择diesel,sqlx之类的框架?
| 框架    | 协程异步async高并发 | 使用难度 | 同时支持Xml/Wrapper/内置增删改查 | logic del逻辑删除插件| page分页插件
| ------ | ------ |------ |------ |------ |------ |
| rbatis | √     | 非常简单   |   √     |    √     |   √     |  
| sqlx   | √     | 难（强依赖宏和 莫名其妙的环境变量）       |   x     |   x     |   x     |  
| diesel | x     | 简单（缺xml支持） |   x     |  x     |  x     |  


##### 和Go语言对比性能(环境（docker）仅供参考)
| 框架     | Mysql（docker） | SQL语句（1万次） | 纳秒/每操作（低越好） | Qps(高越好) |内存消耗（低越好） |
|  ------ | ------ |------ |------ |------ |------ |
| Rust语言-rbatis/tokio  |  1CPU,1G内存    | select count(1) from table;    | 965649 ns/op   |  1035 Qps/s  |  2.1MB   |      
| Go语言-GoMybatis/http   |  1CPU,1G内存   | select count(1) from table;   | 1184503 ns/op  |  844  Qps/s   |  28.4MB  |     


* 使用最通用的json数据结构（基于serde_json）进行传参和通讯
* 高性能，单线程benchmark 可轻松拉起200000 QPS/s（直接返回数据（数据库查询时间损耗0），win10,6 core i7,16GB）  多线程更高 远超go语言版本的GoMyBatis
* 多功能，逻辑删除插件+分页插件+Py风格Sql+基本的Mybatis功能
* 支持future,async await（理论上，假设严格按照async_std/tokio库替代所有io操作，那么并发量可远远超过go语言）
* 日志支持,可自定义具体日志（基于标准库log(独立于任何特定的日志记录库)，日志可选任意第三方库实现）
* 使用百分百的安全代码实现(lib.rs加入了"#![forbid(unsafe_code)]" 禁止不安全的unsafe代码)
* [示例代码（需要Clion导入）](https://github.com/rbatis/rbatis/tree/master/example/src)
* [示例项目（需要Clion导入）](https://github.com/rbatis/abs_admin)

##### 实战(编写Rust后台服务) https://github.com/rbatis/abs_admin

##### 使用方法：添加依赖(Cargo.toml)
``` rust
# add this library,and cargo install

#json支持(必须)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

#日期支持(必须)
chrono = { version = "0.4", features = ["serde"] }

#log日志支持(必须)
log = "0.4"
fast_log="1.2.2"

#BigDecimal支持(可选)
bigdecimal = "0.2"

#rbatis支持，版本保持一致(必须)
rbatis-core = { version = "1.5.8", features = ["all"]}
rbatis =  { version = "1.5.8" } 
rbatis-macro-driver = { version = "1.5.8" }

```

##### 一分钟快速学会， QueryWrapper，常用方法(详见example/crud_test.rs)
```rust
#[macro_use]
extern crate rbatis_macro_driver;
///数据库表模型 CRUDEnable也可以写成 impl CRUDEnable for BizActivity{}
#[derive(CRUDEnable,Serialize, Deserialize, Clone, Debug)]
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
//impl CRUDEnable for BizActivity {
//    type IdType = String;    
//    fn table_name()->String{
//        "biz_activity".to_string()
//    }
//    fn table_fields()->String{
//        "id,name,delete_flag".to_string()
//    }
//}

#[actix_rt::main]
async fn main() {
///rbatis初始化，rbatis是线程安全可使用lazy_static 定义为全局变量
let rb = Rbatis::new();
///连接数据库   
rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
///自定义连接池参数。(可选)
// let mut opt =PoolOptions::new();
// opt.max_size=100;
// rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
///新建的wrapper sql逻辑
let wrapper = rb.new_wrapper()
            .eq("id", 1)                    //sql:  id = 1
            .and()                          //sql:  and 
            .ne("id", 1)                    //sql:  id <> 1
            .in_array("id", &[1, 2, 3])     //sql:  id in (1,2,3)
            .not_in("id", &[1, 2, 3])       //sql:  id not in (1,2,3)
            .like("name", 1)                //sql:  name like 1
            .or()                           //sql:  or
            .not_like("name", "asdf")       //sql:  name not like 'asdf'
            .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")//sql:  create_time between '2020-01-01 00:00:00' and '2020-01-01 00:00:00'
            .group_by(&["id"])              //sql:  group by id
            .order_by(true, &["id", "name"])//sql:  group by id,name
            .check().unwrap();

let activity = BizActivity {
                id: Some("12312".to_string()),
                name: None,
                remark: None,
                create_time: Some(NaiveDateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
            };
///保存
rb.save("",&activity).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

///批量保存
rb.save_batch("", &vec![activity]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

///查询, Option包装，有可能查不到数据则为None
let result: Option<BizActivity> = rb.fetch_by_id("", &"1".to_string()).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

///查询-全部
let result: Vec<BizActivity> = rb.list("").await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1

///批量-查询id
let result: Vec<BizActivity> = rb.list_by_ids("",&["1".to_string()]).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?) 

///自定义查询
let w = rb.new_wrapper().eq("id", "1").check().unwrap();
let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
//Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

///删除
rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

///批量删除
rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  ) 

///修改
let w = rb.new_wrapper().eq("id", "12312").check().unwrap();
rb.update_by_wrapper("", &activity, &w).await;
//Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ? 
}

///...还有更多方法，请查看crud.rs
```


#### 智能宏映射（新功能）
```rust
    lazy_static! {
     static ref RB:Rbatis=Rbatis::new();
   }

    /// 宏根据方法定义生成执行逻辑，又点类似于 java/mybatis的@select动态sql
    /// RB是本地依赖Rbatis引用的名称,例如  dao::RB, com::xxx::RB....都可以
    /// 第二个参数是标准的驱动sql，注意对应数据库参数mysql为？,pg为$1...
    /// 宏会自动转换函数为  pub async fn select(name: &str) -> rbatis_core::Result<BizActivity> {}
    ///
    #[sql(RB, "select * from biz_activity where id = ?")]
    fn select(name: &str) -> BizActivity {}
    //其他写法： pub async fn select(name: &str) -> rbatis_core::Result<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro() {
        fast_log::log::init_log("requests.log", &RuntimeType::Std);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = select("1").await.unwrap();
        println!("{:?}", a);
    }
```
```rust
    lazy_static! {
     static ref RB:Rbatis=Rbatis::new();
   }

    /// 宏根据方法定义生成执行逻辑，又点类似于 java/mybatis的@select动态sql
    /// RB是本地依赖Rbatis引用的名称,例如  dao::RB, com::xxx::RB....都可以
    /// 第二个参数是标准的驱动sql，注意对应数据库参数mysql为？,pg为$1...
    /// 宏会自动转换函数为  pub async fn select(name: &str) -> rbatis_core::Result<BizActivity> {}
    ///
    #[py_sql(RB, "select * from biz_activity where id = #{name}
                  if name != '':
                    and name=#{name}")]
    fn py_select(name: &str) -> Option<BizActivity> {}
    //其他写法： pub async fn select(name: &str) -> rbatis_core::Result<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select() {
        fast_log::log::init_log("requests.log", &RuntimeType::Std);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select("1").await.unwrap();
        println!("{:?}", a);
    }
```



##### 逻辑删除插件使用(逻辑删除针对Rbatis提供的查询方法和删除方法有效，例如方法 list**(),remove**()，fetch**())
```rust
   let mut rb = init_rbatis().await;
   //rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("delete_flag",1,0)));//自定义已删除/未删除 写法
   rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("delete_flag")));
   rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
           let r = rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
           if r.is_err() {
               println!("{}", r.err().unwrap().to_string());
   }
```


##### 分页插件使用
```rust
        let mut rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        //框架默认RbatisPagePlugin，如果需要自定义的话需要结构体 必须实现impl PagePlugin for Plugin***{}，例如：
        //rb.page_plugin = Box::new(RbatisPagePlugin {});

        let req = PageRequest::new(1, 20);//分页请求，页码，条数
        let wraper= rb.new_wrapper()
                    .eq("delete_flag",1)
                    .check()
                    .unwrap();
        let data: Page<BizActivity> = rb.fetch_page_by_wrapper("", &wraper,  &req).await.unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());

//2020-07-10T21:28:40.036506700+08:00 INFO rbatis::rbatis - [rbatis] Query ==> SELECT count(1) FROM biz_activity  WHERE delete_flag =  ? LIMIT 0,20
//2020-07-10T21:28:40.040505200+08:00 INFO rbatis::rbatis - [rbatis] Args  ==> [1]
//2020-07-10T21:28:40.073506+08:00 INFO rbatis::rbatis - [rbatis] Total <== 1
//2020-07-10T21:28:40.073506+08:00 INFO rbatis::rbatis - [rbatis] Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity  WHERE delete_flag =  ? LIMIT 0,20
//2020-07-10T21:28:40.073506+08:00 INFO rbatis::rbatis - [rbatis] Args  ==> [1]
//2020-07-10T21:28:40.076506500+08:00 INFO rbatis::rbatis - [rbatis] Total <== 5
```
```json
{
	"records": [{
		"id": "12312",
		"name": "null",
		"pc_link": "null",
		"h5_link": "null",
		"pc_banner_img": "null",
		"h5_banner_img": "null",
		"sort": "null",
		"status": 1,
		"remark": "null",
		"create_time": "2020-02-09T00:00:00+00:00",
		"version": 1,
		"delete_flag": 1
	}],
	"total": 5,
	"size": 20,
	"current": 1,
	"serch_count": true
}
```


##### py风格sql语法Example
``` python
//执行到远程mysql 并且获取结果。支持serde_json可序列化的任意类型
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
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
```

#### 日志系统(这里举例使用fast_log)
``` rust
 //main函数加入
 use log::{error, info, warn};
 fn  main(){
      fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
      info!("print data");
 }
```


#### 自定义连接池大小，超时，活跃连接数等等

```rust
use rbatis_core::db::PoolOptions;

pub async fn init_rbatis() -> Rbatis {
let rb = Rbatis::new();
let mut opt = PoolOptions::new();
opt.max_size = 20;
rb.link_opt("mysql://root:123456@localhost:3306/test", &opt).await.unwrap();
}
```


#### XML使用方法
``` rust
/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
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
fn main() {
    async_std::task::block_on(
           async move {
               fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
               let mut rb = Rbatis::new();
               rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
               //xml数据建议以 XXMapper.xml 的格式存储管理
               rb.load_xml("test", r#"<?xml version="1.0" encoding="UTF-8"?>
   <!DOCTYPE mapper PUBLIC "-//mybatis.org//DTD Mapper 3.0//EN"
           "https://raw.githubusercontent.com/zhuxiujia/Rbatis/master/rbatis-mapper.dtd">
   <mapper>
       <result_map id="BaseResultMap" table="biz_activity">
           <id column="id"/>
           <result column="name" lang_type="string"/>
           <result column="pc_link" lang_type="string"/>
           <result column="h5_link" lang_type="string"/>
           <result column="pc_banner_img" lang_type="string"/>
           <result column="h5_banner_img" lang_type="string"/>
           <result column="sort" lang_type="string"/>
           <result column="status" lang_type="number"/>
           <result column="remark" lang_type="string"/>
           <result column="version" lang_type="number" version_enable="true"/>
           <result column="create_time" lang_type="time"/>
           <result column="delete_flag" lang_type="number" logic_enable="true" logic_undelete="1"
                   logic_deleted="0"/>
       </result_map>
       <select id="select_by_condition">
           <bind name="pattern" value="'%' + name + '%'"/>
           select * from biz_activity
           <where>
               <if test="name != null">and name like #{pattern}</if>
               <if test="startTime != null">and create_time >= #{startTime}</if>
               <if test="endTime != null">and create_time &lt;= #{endTime}</if>
           </where>
           order by create_time desc
           <if test="page != null and size != null">limit #{page}, #{size}</if>
       </select>
   </mapper>"#).unwrap();
   
               let arg = &json!({
               "delete_flag": 1,
               "name": "test",
               "startTime": null,
               "endTime": null,
               "page": 0,
               "size": 20
               });
               let data: Vec<BizActivity> = rb.xml_fetch("", "test", "select_by_condition", arg).await.unwrap();
               println!("{}", serde_json::to_string(&data).unwrap_or("".to_string()));
           }
       )
}
//输出结果
//2020-06-27T03:13:40.422307200+08:00 INFO rbatis::rbatis - [rbatis] >> fetch sql: select * from biz_activity where name like  ? order by create_time desc limit  ? ,  ?   (src\rbatis.rs:198)
//2020-06-27T03:13:40.424307300+08:00 INFO rbatis::rbatis - [rbatis] >> fetch arg:["%test%",0,20]  (src\rbatis.rs:199)
//2020-06-27T03:13:40.446308900+08:00 INFO rbatis::rbatis - [rbatis] << 4  (src\rbatis.rs:234)
//[{"id":"221","name":"test","pc_link":"","h5_link":"","pc_banner_img":null,"h5_banner_img":null,"sort":"0","status":0,"remark":"","create_time":"2020-06-17T20:10:23Z","version":0,"delete_flag":1},{"id":"222","name":"test","pc_link":"","h5_link":"","pc_banner_img":null,"h5_banner_img":null,"sort":"0","status":0,"remark":"","create_time":"2020-06-17T20:10:23Z","version":0,"delete_flag":1},{"id":"223","name":"test","pc_link":"","h5_link":"","pc_banner_img":null,"h5_banner_img":null,"sort":"0","status":0,"remark":"","create_time":"2020-06-17T20:10:23Z","version":0,"delete_flag":1},{"id":"178","name":"test_insret","pc_link":"","h5_link":"","pc_banner_img":null,"h5_banner_img":null,"sort":"1","status":1,"remark":"","create_time":"2020-06-17T20:08:13Z","version":0,"delete_flag":1}]
```

#### 事务支持
``` rust
   async_std::task::block_on(async {
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let tx_id = "1";//事务id号
        rb.begin(tx_id).await.unwrap();
        let v: serde_json::Value = rb.fetch(tx_id, "SELECT count(1) FROM biz_activity;").await.unwrap();
        println!("{}", v.clone());
        rb.commit(tx_id).await.unwrap();
    });
```



### Web框架支持(这里举例actix-web,支持所有基于tokio,async_std的web框架)
``` rust
lazy_static! {
   static ref RB:Rbatis=Rbatis::new();
}

async fn index() -> impl Responder {
    let v:Result<i32,rbatis_core::Error> = RB.fetch("", "SELECT count(1) FROM biz_activity;").await;
    HttpResponse::Ok().body(format!("count(1)={}",v.unwrap_or(0)))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //日志
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    //链接数据库
    RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
    //http路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
```
### 支持数据结构列表
| 数据库    | 已支持 |
| ------ | ------ |
| Option                   | √     | 
| Vec                      | √     |  
| HashMap                      | √     |  
| Slice                    | √     |   
| i32,i64,f32,f64,bool,String...more rust type   | √     |  
| NativeDateTime           | √     |  
| BigDecimal               | √     |
| serde_json::Value...more serde type         | √     |

### 支持数据库类型√已支持.进行中
| 数据库    | 已支持 |
| ------ | ------ |
| Mysql            | √     |   
| Postgres         | √     |  
| Sqlite           | √     |  
| TiDB             | √     |
| CockroachDB      | √     |

### 平台测试通过
| 平台    | 已支持 |
| ------ | ------ |
| Linux                   | √     | 
| Apple/MacOS             | √     |  
| Windows               | √     |


### 进度表-按照顺序实现
| 功能    | 已支持 |
| ------ | ------ |
| CRUD(内置CRUD模板(内置CRUD支持逻辑删除插件))                  | √     |
| LogSystem(日志组件)                                          | √     | 
| Tx(事务/事务嵌套/注解声明式事务)                                | √     |   
| Py(在SQL中使用和xml等价的类python语法)                         | √     | 
| SlowSqlCount(内置慢查询日志分析)                              | √     | 
| async/await支持                                             | √     | 
| PagePlugin(分页插件)                                         | √     |
| LogicDelPlugin(逻辑删除插件)                                 | √    |
| DataBaseConvertPlugin(数据库表结构转换为配置插件)               | x     | 
| web(可视化Web UI)                                            | x     |  


### 基准测试benchmark (测试平台 win10,6 core i7,16GB)
#### 分步骤压测
``` 
//sql构建性能  Example_ActivityMapper.xml -> select_by_condition
操作/纳秒nano/op: 0.202 s,each:2020 nano/op
事务数/秒 TPS: 495049.50495049503 TPS/s

//查询结果解码性能 decode/mysql_json_decoder  ->  bench_decode_mysql_json
操作/纳秒nano/op: 0.24 s,each:2400 nano/op
事务数/秒 TPS: 416666.6666666667 TPS/s

//综合性能约等于
操作/纳秒nano/op:   4420 nano/op 
事务数/秒 TPS: 200000  TPS/s
``` 

* 结论： 假设IO耗时为0的情况下，仅单线程 便有高达20万QPS/TPS，性能也是go语言版、java版 等等有GC暂停语言的 几倍以上性能




### FAQ 常见问题
* 日期时间和BigDecimal支持？<br/>
已支持chrono::NaiveDateTime和bigdecimal::BigDecimal
* async await支持？<br/>
已同时支持async_std和tokio
* postgres 的stmt使用$1,$2而不是mysql的?,那么是否需要特殊处理？<br/>
不需要，因为rbatis 99%的api使用#{}描述参数变量，只需要写具体参数名称，不需要对应数据库的符号。
* oracle数据库驱动支持？<br/>
不支持，应该坚持去IOE
* 直接使用驱动依赖项目里哪个库？<br/>
应该使用rbatis-core， Cargo.toml 加入 rbatis-core = "*"
* 如何选择运行时是tokio还是async_std？<br/>
```rust
# Cargo.toml 加入 
rbatis-core = { features = ["runtime-async-std","all-type"]}
# 或者Cargo.toml 加入 
# rbatis-core = { features = ["runtime-tokio","all-type"]}
```


### 和Rbatis相关项目
* Log日志库 https://github.com/rbatis/fast_log
* Rbatis/Core  https://github.com/rbatis/rbatis/tree/master/rbatis-core


# TODO 即将到来的特性


## 为了称心如意的ORM框架，您的支持永远是我们的动力，迫切欢迎微信捐赠支持我们 ~或者~右上角点下star
![Image text](https://zhuxiujia.github.io/gomybatis.io/assets/wx_account.jpg)
