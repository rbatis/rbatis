
#### Rbatis a Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
#### Rbatis 是一个高性能的,动态sql,低内存开销的,无runtime,无GC,并发安全的  数据库 ORM框架
[![Build Status](https://travis-ci.org/zhuxiujia/RustMybatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/RustMybatis)
* 简洁如同Java的Mybatis,比肩C/C++的极高性能
* 基于抽象语法树 生成动态的sql语句,避免运行中使用递归，达到高性能的目的
* 内部使用serde_json编码和解码数据库返回数据，以达到最高兼容性
* 吸收go语言版GoMybatis经验，结合Rust语言开辟高效率开发方案和生产级别实践

#### 支持数据库类型
* Mysql
* PostgreSQL
* TiDB(NewSql)
* CockroachDB(NewSql)




### 进度表
| ast    | xmlLoader | expressEngines | sqlDecoder | logSystem | dataSourceRouter |templeteDecoder |
| ------ | ------ | ------ | ------ | ------ | ------ |------ |
| √      | √      | √      | *      | *      | *      | *     |

### 动态表达式性能(测试平台 win10,6 core i7,16GB)
<pre>
     bench: '1 <= 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 >= 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 > 2'   parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     bench: '1 < 2'   parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0) 
     bench: ''1'+'1'' parser_test::Bench_Parser ... bench:          118 ns/iter (+/- 1)
</pre>
