RDBC

* support zero copy serde-ser/de

Database -> bytes ->rbs::Value-> Struct(User Define)
Struct(User Define) -> rbs::ValueRef -> ref clone() -> Database