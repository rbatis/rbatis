use crate::db::db_adapter::DataDecoder;
use crate::db::DriverType;
use crate::Result;
use rbson::Bson;

///the stmt replace str convert
pub trait StmtConvert {
    fn stmt_convert(&self, index: usize, item: &mut String);
}

#[macro_export]
macro_rules! push_index {
    ($n:expr,$new_sql:ident,$index:expr) => {{
        let num = $index / $n;
        $new_sql.push((num + 48) as u8 as char);
        $index % $n
    }};
    ($index:ident,$new_sql:ident) => {
        if $index >= 0 && $index < 10 {
            $new_sql.push(($index + 48) as u8 as char);
        } else if $index >= 10 && $index < 100 {
            let $index = push_index!(10, $new_sql, $index);
            let $index = push_index!(1, $new_sql, $index);
        } else if $index >= 100 && $index < 1000 {
            let $index = push_index!(100, $new_sql, $index);
            let $index = push_index!(10, $new_sql, $index);
            let $index = push_index!(1, $new_sql, $index);
        } else if $index >= 1000 && $index < 10000 {
            let $index = push_index!(1000, $new_sql, $index);
            let $index = push_index!(100, $new_sql, $index);
            let $index = push_index!(10, $new_sql, $index);
            let $index = push_index!(1, $new_sql, $index);
        } else {
            use std::fmt::Write;
            $new_sql
                .write_fmt(format_args!("{}", $index))
                .expect("a Display implementation returned an error unexpectedly");
        }
    };
}

impl StmtConvert for DriverType {
    fn stmt_convert(&self, index: usize, item: &mut String) {
        match &self {
            DriverType::Postgres => {
                item.push('$');
                let index = index + 1;
                push_index!(index, item);
            }
            DriverType::Mysql => {
                item.push('?');
            }
            DriverType::Sqlite => {
                item.push('?');
            }
            DriverType::Mssql => {
                item.push('@');
                item.push('p');
                let index = index + 1;
                push_index!(index, item);
            }
            DriverType::None => {
                panic!("[rbatis] un support none for driver type!")
            }
        }
    }
}

///json convert
pub trait JsonCodec {
    /// to an json value
    fn try_to_bson(self) -> Result<Bson>;
}

///json convert
pub trait RefJsonCodec {
    /// to an json value
    fn try_to_bson(&self, decoder: &dyn DataDecoder) -> Result<Bson>;
}

///result convert
pub trait ResultCodec<T> {
    fn into_result(self) -> Result<T>;
}

#[macro_export]
macro_rules! to_bson_macro {
    ($r:ident) => {{
        if $r.is_some() {
            rbson::bson!($r.unwrap())
        } else {
            rbson::Bson::Null
        }
    }};
}
