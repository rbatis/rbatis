use std::fmt::{Debug, Display, Formatter};

pub mod db;
pub mod decode;
pub mod encode;
pub mod ext;
pub mod io;
pub mod rt;
pub mod common;

#[derive(Debug)]
pub enum Error {
    E(String),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::E(e) => std::fmt::Display::fmt(&e, f),
            Error::Io(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::E(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(arg: std::io::Error) -> Self {
        Error::E(arg.to_string())
    }
}

impl Error {
    #[allow(dead_code)]
    #[inline]
    pub fn protocol(err: impl Display) -> Self {
        Error::E(err.to_string())
    }
}

// Format an error message as a `Protocol` error
#[macro_export]
macro_rules! err_protocol {
    ($expr:expr) => {
        $crate::Error::E($expr.into())
    };

    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::E(format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_ser_ref() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
        }
        let a = A {
            name: "sss".to_string(),
        };
        let v = rbs::to_value_ref(&a).unwrap();
        println!("{:?}", v);

        let mut m = HashMap::new();
        m.insert(1, 2);
        let v = rbs::to_value_ref(&m).unwrap();
        println!("{:?}", v);

        let v = rbs::to_value(a).unwrap();
        println!("v: {}", v);
        let s: A = rbs::from_value(v).unwrap();
        println!("s:{:?}", s);
    }

    #[test]
    fn test_ser() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
            pub i32: i32,
            pub u32: u32,
            pub i64: i64,
            pub u64: u64,
        }
        let buf = rbs::to_vec(&A {
            name: "s".to_string(),
            i32: i32::MAX,
            u32: u32::MAX,
            i64: i64::MAX,
            u64: u64::MAX,
        })
        .unwrap();
        let v: rbs::Value = rbs::read_value(&mut &buf[..]).unwrap();
        println!("{}", v);

        let v: A = rbs::decode::from_slice(&buf).unwrap();
        println!("{:?}", v);
    }
}
