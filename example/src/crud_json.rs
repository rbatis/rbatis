#[macro_use]
extern crate rbatis;

pub mod init;

use log::LevelFilter;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbs::{from_value, to_value, Value};
use crate::init::{init_db};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizUser {
    pub id: Option<String>,
    pub account: Option<Account>,
}
crud!(BizUser{});

#[derive(Clone, Debug, serde::Serialize, Default)]
pub struct Account {
    pub id: Option<String>,
    pub name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
        pub struct AccountProxy {
            pub id: Option<String>,
            pub name: Option<String>,
        }
        impl From<AccountProxy> for Account {
            fn from(value: AccountProxy) -> Self {
                Account {
                    id: value.id,
                    name: value.name,
                }
            }
        }
        let z = rbs::Value::deserialize(deserializer)?;
        match z {
            Value::String(v) => {
                let account: Account = serde_json::from_str(&v).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(account)
            }
            _ => {
                let account: AccountProxy = serde_json::from_str(&z.to_string()).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(account.into())
            }
        }
    }
}

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
        .expect("rbatis init fail");
    let rb = init_db().await;

    let user = BizUser {
        id: Some("1".to_string()),
        account: Some(Account { id: Some("2".to_string()), name: Some("xxx".to_string()) }),
    };
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _ = SqliteTableSync::default().sync(rb.acquire().await.unwrap(), to_value!(&user), "biz_user").await;
    fast_log::LOGGER.set_level(LevelFilter::Info);

    let v = BizUser::insert(&mut rb.clone(), &user).await;
    println!("insert:{:?}", v);

    let users = BizUser::select_by_column(&mut rb.clone(), "id", "1").await;
    println!("select:{}", to_value!(users));
}
