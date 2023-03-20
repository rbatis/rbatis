use crate::table_sync::TableSync;
use crate::Error;
use futures_core::future::BoxFuture;
use rbs::Value;
use std::ops::Index;
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::{Error, RBDCString};
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;

pub struct SqliteTableSync {
    pub sql_id: String,
}

impl Default for SqliteTableSync {
    fn default() -> Self {
        Self {
            sql_id: " PRIMARY KEY NOT NULL ".to_string(),
        }
    }
}

fn type_str(v: &Value) -> &'static str {
    match v {
        Value::Null => "NULL",
        Value::Bool(_) => "BOOLEAN",
        Value::I32(_) => "INTEGER",
        Value::I64(_) => "INT8",
        Value::U32(_) => "INTEGER",
        Value::U64(_) => "INT8",
        Value::F32(_) => "DOUBLE",
        Value::F64(_) => "DOUBLE",
        Value::String(v) => {
            if Date::is(&v) != "" {
                "TEXT"
            } else if DateTime::is(&v) != "" {
                "TEXT"
            } else if Time::is(&v) != "" {
                "TEXT"
            } else if Timestamp::is(&v) != "" {
                "INT8"
            } else if Decimal::is(&v) != "" {
                "TEXT"
            } else if Uuid::is(&v) != "" {
                "TEXT"
            } else {
                "TEXT"
            }
        },
        Value::Binary(_) => "BLOB",
        Value::Array(_) => "BLOB",
        Value::Map(_) => {
            "BLOB"
        }
    }
}

impl TableSync for SqliteTableSync {
    fn sync(
        &self,
        mut rb: RBatisConnExecutor,
        table: Value,
        name: &str,
    ) -> BoxFuture<Result<(), Error>> {
        let name = name.to_owned();
        Box::pin(async move {
            match table {
                Value::Map(m) => {
                    let mut sql_create = format!("CREATE TABLE {} ", name);
                    let mut sql_column = format!("");
                    for (k, v) in &m {
                        let k = k.as_str().unwrap_or_default();
                        sql_column.push_str(k);
                        sql_column.push_str(" ");
                        sql_column.push_str(type_str(&v));
                        if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                            sql_column.push_str(&self.sql_id);
                        }
                        sql_column.push_str(",");
                    }
                    if sql_column.ends_with(",") {
                        sql_column = sql_column.trim_end_matches(",").to_string();
                    }
                    sql_create = sql_create + &format!("({});", sql_column);
                    let result_create = rb.exec(&sql_create, vec![]).await;
                    match result_create {
                        Ok(_) => {}
                        Err(e) => {
                            if e.to_string().contains("already exists") {
                                for (k, v) in &m {
                                    let k = k.as_str().unwrap_or_default();
                                    let mut id_key = "";
                                    if k.eq("id") || v.as_str().unwrap_or_default() == "id" {
                                        id_key = &self.sql_id;
                                    }
                                    match rb
                                        .exec(
                                            &format!(
                                                "alter table {} add {} {} {};",
                                                name,
                                                k,
                                                type_str(&v),
                                                id_key
                                            ),
                                            vec![],
                                        )
                                        .await
                                    {
                                        Ok(_) => {}
                                        Err(e) => {
                                            if e.to_string().contains("duplicate column") {
                                                continue;
                                            }
                                            return Err(e);
                                        }
                                    }
                                }
                                return Ok(());
                            }
                            return Err(e);
                        }
                    }
                    Ok(())
                }
                _ => Err(Error::from("table not is an struct or map!")),
            }
        })
    }
}
