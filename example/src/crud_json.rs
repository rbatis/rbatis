#[macro_use]
extern crate rbatis;

pub mod model;

use log::LevelFilter;
use serde::de::Error;
use serde::Deserializer;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbs::{from_value, to_value, Value};
use crate::model::{init_db};

#[derive(Clone, Debug, serde::Serialize)]
pub struct BizUser {
    pub id: Option<String>,
    pub account: Option<Account>,
}
crud!(BizUser{});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Account {
    pub id: Option<String>,
    pub name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BizUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct BizUserZ {
            pub id: Option<String>,
            pub account: Value,
        }
        let z = BizUserZ::deserialize(deserializer)?;
        match z.account {
            Value::Null => {
                let account: Option<Account> = from_value(z.account).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(Self {
                    id: z.id,
                    account: account,
                })
            }
            Value::String(v) => {
                let account: Option<Account> = serde_json::from_str(&v).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(Self {
                    id: z.id,
                    account: account,
                })
            }
            Value::Binary(v) => {
                let account: Option<Account> = serde_json::from_slice(&v).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(Self {
                    id: z.id,
                    account: account,
                })
            }
            Value::Map(_) => {
                let account: Option<Account> = from_value(z.account).map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
                Ok(Self {
                    id: z.id,
                    account: account,
                })
            }
            _ => {
                Err(D::Error::custom("warn type"))
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
