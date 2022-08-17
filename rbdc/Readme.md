RBDC

* an database driver abstract

* support zero copy serde-ser/de

Database -> bytes ->rbs::Value-> Struct(User Define)
Struct(User Define) -> rbs::ValueRef -> ref clone() -> Database

* supported database drivers see [rbatis](https://github.com/rbatis/rbatis)

### how to define my driver?
should impl trait and load driver
* impl trait Driver
* impl trait Connection
* impl trait Statement
* impl trait ResultSet
* impl trait MetaData
* impl trait ConnectOptions
* impl trait Placeholder
