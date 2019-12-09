
#### A ORM formwork Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
#### rbatis 是一个无Gc无虚拟机无运行时Runtime直接编译为机器码,并发安全的  数据库 ORM框架，并且所有数据传值均使用json（serde_json）
[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)


* 因为rust语言没有GC,因此框架理论上能承受极大的并发量，并且是无锁，内存安全，线程安全，低开销的代码
* 性能极高,例子中的select id="select_by_condition"内的代码 单线程可轻松拉起411522.63 TPS/s以上并发（直接返回数据（数据库查询时间损耗0），win10,6 core i7,16GB）  多线程更高 远超go语言版本的GoMyBatis
* 借鉴GoMybatis（Go语言系）以及Mybatis Plus（Java系）的ORM框架经验 例如乐观锁+版本号
* 支持async_std库异步的形式（理论上，假设严格按照async_std库替代所有io操作，那么并发量可远远超过go语言）
* rust版本构建要求 stable:1.9 以上

##### xml案例: xml_string
``` xml
<mapper>
    <result_map id="BaseResultMap">
        <id column="id" property="id"/>
        <result column="name" property="name" lang_type="string"/>
        <result column="pc_link" property="pcLink" lang_type="string"/>
        <result column="h5_link" property="h5Link" lang_type="string"/>
        <result column="remark" property="remark" lang_type="string"/>
        <result column="version" property="version" lang_type="int" version_enable="true"/>
        <result column="create_time" property="createTime" lang_type="time"/>
        <result column="delete_flag" property="deleteFlag" lang_type="int" logic_enable="true" logic_undelete="1" logic_deleted="0"/>
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
##### rust案例:
``` rust
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value};
use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
/**
* 数据库表模型
*/
#[derive(Serialize, Deserialize, Debug, Clone,RbatisMacro)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<i32>,
}

//......
let mut rbatis=Rbatis::new();
rbatis.load_xml("Example_ActivityMapper.xml".to_string(),xml_string);//读取上面的xml
let data_result:Result<serde_json::Value,String>=rbatis.eval("".to_string(), "select_by_condition", &mut json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    }));
println!("[rbatis] result==> {}",data_result.unwrap());

//.......执行输出结果
//[rbatis] Query ==>   select * from biz_activity  order by create_time desc
//[rbatis] result==> [{"create_time":"\"2019-05-27 10:25:41\"","delete_flag":1,"h5_banner_img":"\"http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0\"","h5_link":"\"http://115.220.9.139:8002/newuser/\"","id":"\"dfbdd779-5f70-4b8f-9921-a235a9c75b69\"","name":"\"新人专享\"","pc_banner_img":"\"http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0\"","pc_link":"\"http://115.220.9.139:8002/newuser/\"","remark":"\"\"","sort":"\"\"","status":0,"version":6},{"create_time":"\"2019-05-27 10:25:41\"","delete_flag":1,"h5_banner_img":"\"http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0\"","h5_link":"\"http://115.220.9.139:8002/newuser/\"","id":"\"dfbdd779-5f70-4b8f-9921-c235a9c75b69\"","name":"\"新人专享\"","pc_banner_img":"\"http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0\"","pc_link":"\"http://115.220.9.139:8002/newuser/\"","remark":"\"\"","sort":"\"\"","status":0,"version":6}]
```

### 支持数据库类型
| 数据库    | 已支持 |
| ------ | ------ |
| mysql            | √     |   
| postgres         | √     |  
| TiDB             | x     |
| CockroachDB      | x     |

### 进度表
| 功能    | 已支持 |
| ------ | ------ |
| ast                                     | √     |     
| xmlLoader                               | √     |  
| expressEngines                          | √     |  
| resultDecoder                           | √     |  
| crud_support(CRUD模板)                  | x     |
| page_plug(分页插件)                      | x     |
| security_plug(sql注入拦截插件)           | x     |
| tx(事务/事务嵌套/注解声明式事务)          | x     |  
| logSystem                               | x     |  
| dataSourceRouter                        | x     |  
| web(可独立部署Web UI服务)                | x     |  


### 性能测算 (测试平台 win10,6 core i7,16GB)
#### 分步骤压测
``` 
（sql构建性能）Example_ActivityMapper.xml -> select_by_condition
///use Time: 0.202 s,each:2020 nano/op
///use TPS: 495049.50495049503 TPS/s
（查询结果解码性能）Example_ActivityMapper.xml -> select_by_condition
///use Time: 0.042 s,each:420 nano/op
///use TPS: 2380952.3809523806 TPS/s
（综合性能）
约为:
操作/纳秒   2440    nano/op 
事务数/秒   411522.63 TPS/s
``` 


#### (原生Rust代码数值运算约等于 1 ns/iter,字符串运算约等于100 ns/iter)
<pre>
     bench: '1 <= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 >= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 > 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 < 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0) 
     bench: ''1'+'1'' parser_test::bench_parser ... bench:          118 ns/iter (+/- 1)
</pre>
