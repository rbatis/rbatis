 init pool by DBPoolOptions and Pool
 
* for example:
```
 use rbatis::RBatis;
 use rbdc_pool_deadpool::DeadPool;
 use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
 let rb=RBatis::new();

 let opts=SqliteConnectOptions::new();
 let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, DeadPool>(SqliteDriver{},opts);
 ```
