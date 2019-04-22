
#### RustMybatis a Rustlang-based,dynamic sql, no Runtime,no GC, low Memory use,High Performance orm Framework.
[![Build Status](https://travis-ci.org/zhuxiujia/RustMybatis.svg?branch=master)](https://travis-ci.org/zhuxiujia/RustMybatis)


### RustMybatis 是一个高性能的,动态sql,低内存开销的,没有运行时,没有GC,并发安全的  数据库 ORM框架

* 基于抽象语法树 生成动态的sql语句,避免运行中使用递归，达到高性能的目的


### 进度表
| ast    | xmlLoader | expressEngines | sqlDecoder | logSystem | dataSourceRouter |templeteDecoder |
| ------ | ------ | ------ | ------ | ------ | ------ |------ |
| √      | √      | √      | *      | *      | *      | *     |

### 动态表达式性能(测试平台 win10,6 core i7,16GB mem)
<pre>识别执行 '1 <= 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     识别执行 '1 >= 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     识别执行 '1 > 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0)
     识别执行 '1 < 2'  parser_test::Bench_Parser ... bench:          21 ns/iter (+/- 0) </pre>


#