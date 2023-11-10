 init pool by DBPoolOptions and Pool
 
* for example:
```
 use rbatis::RBatis;
 use rbdc_pool_mobc::MobcPool;
 use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
 let rb=RBatis::new();

 let opts=SqliteConnectOptions::new();
 let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, MobcPool>(SqliteDriver{},opts);
 ```
