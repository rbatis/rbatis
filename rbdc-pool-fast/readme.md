## rbdc driver connection pool based on [FastPool](https://crates.io/crates/fast_pool)

### way fast_pool?

* fast get() method performance
```log
//windows:
//---- bench_pool stdout ----
//Time: 4.0313ms ,each:40 ns/op
//QPS: 24749412 QPS/s
//macos:
//---- bench_pool stdout ----
// Time: 6.373708ms ,each:63 ns/op
// QPS: 15683710 QPS/s
```
 
* for example:
```
 use rbatis::RBatis;
 use rbdc_pool_fast::FastPool;
 use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
 let rb=RBatis::new();

 let opts=SqliteConnectOptions::new();
 let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, FastPool>(SqliteDriver{},opts);
 ```
