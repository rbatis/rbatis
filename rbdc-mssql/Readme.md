RBDC

* an database driver abstract

* support zero copy serde-ser/de

Database -> bytes ->rbs::Value-> Struct(User Define)
Struct(User Define) -> rbs::ValueRef -> ref clone() -> Database

### how to define my driver?
should impl trait and load driver
* impl trait rbdc::db::{Driver, MetaData, Row, Connection, ConnectOptions, Placeholder};
