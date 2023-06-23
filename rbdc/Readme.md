RBDC driver abstract

* an database driver abstract for rbatis
* supported database drivers see [rbatis](https://github.com/rbatis/rbatis)

### how to define my driver to support rbdc driver?
should impl trait and load driver
* impl trait rbdc::db::{Driver, MetaData, Row, Connection, ConnectOptions, Placeholder};