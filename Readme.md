
#### A ORM formwork Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
#### rbatis 是一个高性能的,动态sql,低内存开销的,无runtime,无GC,并发安全的  数据库 ORM框架，并且所有数据传值均使用json（serde_json）
[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)


* 因为rust语言没有GC,因此框架理论上能承受极大的并发量，并且是无锁，内存安全，线程安全，低开销的代码
* 借鉴GoMybatis（Go语言系）以及Mybatis Plus（Java系）的ORM框架经验
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


* 简洁如同Java的Mybatis,比肩C/C++的极高性能
* 可拓展的sql解析执行引擎
* 基于抽象语法树 生成动态的sql语句,避免运行中使用递归，达到高性能的目的
* 使用宏+serde_json解析对象，解析性能高达 500ns/iter
* 内部使用serde_json编码和解码数据库返回数据，以达到最高兼容性
* 吸纳众多ORM框架经验，例如乐观锁+版本号+模板

#### 支持数据库类型
* Mysql
* PostgreSQL
* TiDB(NewSql)
* CockroachDB(NewSql)


### 支持数据库类型
| mysql    | postgres | TiDB    | CockroachDB |
| ------ | ------ | ------ | ------ |
| √      | √      | *      | *      |

### 进度表
| ast    | crud_support(CRUD模板)    | xmlLoader | expressEngines | resultDecoder | logSystem | dataSourceRouter |
| ------ | ------ | ------ | ------ | ------ | ------ | ------ |
| √      | *      | √      | √      | √      | *      | *      |

### 动态运算表达式性能(测试平台 win10,6 core i7,16GB)(原生Rust代码数值运算约等于 1 ns/iter,字符串运算约等于100 ns/iter)
<pre>
     bench: '1 <= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 >= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 > 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 < 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0) 
     bench: ''1'+'1'' parser_test::bench_parser ... bench:          118 ns/iter (+/- 1)
</pre>
