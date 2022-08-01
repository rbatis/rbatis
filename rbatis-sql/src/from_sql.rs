use rbs::Value;
use crate::ops::{AsSql, AsProxy};


impl AsSql for Value{
    fn as_sql(&self) -> String {
        match self{
            Value::String(s) => { s.to_owned() }
            _ => {self.to_string()}
        }
    }
}

impl AsSql for &Value{
    fn as_sql(&self) -> String {
        match self{
            Value::String(s) => { s.to_owned() }
            _ => {self.to_string()}
        }
    }
}

impl AsSql for &&Value{
    fn as_sql(&self) -> String {
        match self{
            Value::String(s) => { s.to_owned() }
            _ => {self.to_string()}
        }
    }
}


macro_rules! to_sql {
    ([$($ty:ty)*]) => {
$(impl AsSql for $ty{
    fn as_sql(&self) -> String {
        self.to_string()
    }
})*
    };
}

to_sql!([String &String &&String]);
to_sql!([&str &&str]);
to_sql!([i8 i16 i32 i64 isize]);
to_sql!([u8 u16 u32 u64 usize]);
to_sql!([f32 f64]);