
#### A ORM formwork Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
#### rbatis 是一个无GC无虚拟机无运行时Runtime直接编译为机器码,并发安全的  数据库 ORM框架，并且所有数据传值均使用json（serde_json）
[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)


* 极少使用锁，内存安全，线程安全，低开销，为rust语言没有GC,因此框架理论上能承受极大的并发量
* 使用最通用的json数据结构（基于serde_json）进行传参和通讯
* 性能极高，例子中的select id="select_by_condition"内的代码 单线程基准测试benchmark 可轻松拉起200000 QPS/s（直接返回数据（数据库查询时间损耗0），win10,6 core i7,16GB）  多线程更高 远超go语言版本的GoMyBatis
* 多功能，借鉴GoMybatis（Go语言系）以及Mybatis Plus（Java系）的ORM框架经验 例如乐观锁插件+逻辑删除插件+分页插件+WebUI
* 支持async_std库协程，兼容async await（理论上，假设严格按照async_std库替代所有io操作，那么并发量可远远超过go语言）
* 基于Rust语言稳定版构建，要求 stable:1.9 以上
* 内置日志输出,可自定义具体日志（基于标准库log(独立于任何特定的日志记录库)，标准库日志可选任意第三方库实现，类似于java的SLF4j）

##### xml代码Example
``` xml
<mapper>
    <result_map id="BaseResultMap">
        <id column="id" property="id"/>
        <result column="name" property="name" lang_type="string"/>
        <result column="pc_link" property="pcLink" lang_type="string"/>
        <result column="h5_link" property="h5Link" lang_type="string"/>
        <result column="remark" property="remark" lang_type="string"/>
        <result column="version" property="version" lang_type="number" version_enable="true"/>
        <result column="create_time" property="createTime" lang_type="time"/>
        <result column="delete_flag" property="deleteFlag" lang_type="number" logic_enable="true" logic_undelete="1" logic_deleted="0"/>
    </result_map>
    <select id="select_by_condition" result_map="BaseResultMap">
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
</mapper>
``` 
##### rust代码Example:
##### yaml加入以下代码
```$xslt
log = "0.4"
log4rs = "0.8.3"
```
#### rust main.rs 加入以下代码
``` rust
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value, Number};
/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<i32>,
}

fn main() {
//1 启用日志(可选，不添加则不加载日志库)
log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//2 初始化rbatis
let mut rbatis = Rbatis::new();
//3 加载数据库url name 为空，则默认数据库
rbatis.load_db_url("".to_string(), "mysql://root:TEST@localhost:3306/test");
//4 加载xml配置，读取上面的xml
rbatis.load_xml("Example_ActivityMapper.xml".to_string(),xml_string);
let data_result: Vec<Activity> =rbatis.eval("".to_string(), "select_by_condition", &mut json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    })).unwrap();
println!("[rbatis] result==> {:?}",data_result);
}
//输出结果
2020-01-10T10:28:54.437167+08:00 INFO rbatis::core::rbatis - [rbatis] Query ==>  Example_ActivityMapper.xml.select_by_condition: select * from biz_activity  order by create_time desc
2020-01-10T10:28:54.437306+08:00 INFO rbatis::core::rbatis - [rbatis][args] ==>  Example_ActivityMapper.xml.select_by_condition: 
2020-01-10T10:28:54.552097+08:00 INFO rbatis::core::rbatis - [rbatis] ReturnRows <== 2
//[rbatis] result==> [Activity { id: Some("\"dfbdd779-5f70-4b8f-9921-a235a9c75b69\""), name: Some("\"新人专享\""), version: Some(6) }, Activity { id: Some("\"dfbdd779-5f70-4b8f-9921-c235a9c75b69\""), name: Some("\"新人专享\""), version: Some(6) }]
```
### 自定义动态数据源路由
``` rust
    rbatis.load_db_url("".to_string(), "mysql://root:TEST@localhost:3306/test");//默认配置
    rbatis.load_db_url("read".to_string(), "mysql://root:TEST@localhost:3306/test");//只读库
    rbatis.load_db_url("write".to_string(), "postgres://root:TEST@localhost:3306/test");//只写库
    rbatis.router_func = |id| -> String{
        info!("[rbatis]匹配路由key  ====>  {}",id);
        //例如：你可以自定义读写分离
        if id.contains("select"){
            //info!("select开头 加载读路由配置");
            return "read".to_string();
        }else{
            //info!("非select开头 加载写路由配置");
            // return "write".to_string();
        }
        return "".to_string();
    };
```


### 支持数据库类型√已支持.进行中
| 数据库    | 已支持 |
| ------ | ------ |
| Mysql            | √     |   
| Postgres         | √     |  
| TiDB             | .     |
| CockroachDB      | .     |

### 进度表-按照顺序实现
| 功能    | 已支持 |
| ------ | ------ |
| XMLAstTree（xml抽象语法树)                                   | √     |     
| XmlLoader（xml读取器)                                        | √     |  
| ExpressEngines（表达式执行引擎)                              | √     |  
| ResultDecoder（查询结果解码器-基于serde_json)                | √     |  
| YAMLAstTree（类似python/yaml缩进风格的抽象语法树)             | √     |   
| CRUD(内置CRUD模板(内置CRUD支持乐观锁/逻辑删除))               | √     |
| LogSystem(日志组件)                                          | √     | 
| LogicDelPlugin(逻辑删除插件)                                 | √     |
| VersionLockPlugin(乐观锁插件,防止并发修改数据)                | √     |
| PagePlugin(分页插件)                                         | √     |
| DataSourceRouter(动态数据源路由)                             | √     |  
| Tx(事务/事务嵌套/注解声明式事务)                              | x     |   
| JoinQueryPlugin(组合查询插件，对关联查询进行分解代替join)     | x     |
| web(可视化Web UI)                                            | x     |  


### 基准测试benchmark (测试平台 win10,6 core i7,16GB)
#### 分步骤压测

* sql构建性能  Example_ActivityMapper.xml -> select_by_condition
``` 
操作/纳秒nano/op: 0.202 s,each:2020 nano/op
事务数/秒 TPS: 495049.50495049503 TPS/s
``` 

* 查询结果解码性能 decode/mysql_json_decoder  ->  bench_decode_mysql_json
``` 
操作/纳秒nano/op: 0.24 s,each:2400 nano/op
事务数/秒 TPS: 416666.6666666667 TPS/s
``` 

* 综合性能约等于
``` 
操作/纳秒nano/op:   4420 nano/op 
事务数/秒 TPS: 200000  TPS/s
``` 

* 结论： 即便如此，仅 单线程 便有高达20万QPS/TPS，性能也是go语言版、java版 等等有GC暂停语言的 几倍以上性能

         
## 欢迎右上角点下 star 或者 微信 捐赠 和 赞助~
![Image text](https://zhuxiujia.github.io/gomybatis.io/assets/wx_account.jpg)
