RDBC

* support zero copy serde-ser/de

Database -> bytes ->rbmp::Value-> Struct(User Define)
Struct(User Define) -> rbmp::ValueRef -> ref clone() -> Database