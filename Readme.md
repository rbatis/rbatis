
#### A ORM formwork Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
#### rbatis 是一个高性能的,动态sql,低内存开销的,无runtime,无GC,并发安全的  数据库 ORM框架
[![Build Status](https://travis-ci.org/zhuxiujia/rbatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/rbatis)


##### xml案例:
``` xml
<mapper>
    <resultMap id="BaseResultMap">
        <id column="id" property="id"/>
        <result column="name" property="name" langType="string"/>
        <result column="pc_link" property="pcLink" langType="string"/>
        <result column="h5_link" property="h5Link" langType="string"/>
        <result column="remark" property="remark" langType="string"/>
        <result column="version" property="version" langType="int" version_enable="true"/>
        <result column="create_time" property="createTime" langType="time.Time"/>
        <result column="delete_flag" property="deleteFlag" langType="int" logic_enable="true" logic_undelete="1" logic_deleted="0"/>
    </resultMap>

    <insertTemplete tables="biz_activity" />
    <insertTemplete tables="biz_activity" id="InsertTempleteBatch"/>
    <selectTemplete tables="biz_activity" wheres="name?name = #{name}" columns=""/>
    <updateTemplete tables="biz_activity" sets="name?name = #{name}" wheres="name?name = #{name}"/>
    <deleteTemplete tables="biz_activity" wheres="name?name = #{name}"/>

    <select id="selectByCondition" resultMap="BaseResultMap">
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
| ast    | xmlLoader | expressEngines | sqlDecoder | logSystem | dataSourceRouter |templeteDecoder |
| ------ | ------ | ------ | ------ | ------ | ------ |------ |
| √      | √      | √      | √      | *      | *      | *     |

### 动态运算表达式性能(测试平台 win10,6 core i7,16GB)(原生Rust代码数值运算约等于 1 ns/iter,字符串运算约等于100 ns/iter)
<pre>
     bench: '1 <= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 >= 2'  parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 > 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 < 2'   parser_test::bench_parser ... bench:          21 ns/iter (+/- 0) 
     bench: ''1'+'1'' parser_test::bench_parser ... bench:          118 ns/iter (+/- 1)
</pre>
