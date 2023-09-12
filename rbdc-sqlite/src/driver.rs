use crate::SqliteConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver, Placeholder};
use rbdc::Error;

#[derive(Debug)]
pub struct SqliteDriver {}

impl Driver for SqliteDriver {
    fn name(&self) -> &str {
        "sqlite"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let opt: SqliteConnectOptions = url.parse()?;
            let conn = opt.connect().await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt: &SqliteConnectOptions = opt.downcast_ref().unwrap();
        Box::pin(async move {
            let conn = opt.connect().await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(SqliteConnectOptions::default())
    }
}

impl Placeholder for SqliteDriver {
    fn exchange(&self, sql: &str) -> String {
        sql.to_string()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_default() {}
}
// #[cfg(test)]
// mod test {
//     use crate::driver::SqliteDriver;
//     use crate::SqliteConnectOptions;
//     use rbdc::block_on;
//     use rbdc::db::{ConnectOptions, Driver};
//     use rbdc::decimal::Decimal;
//     use rbdc::pool::Pool;
//     use rbs::Value;
//     use std::fs::File;
//
//     #[test]
//     fn test_sqlite_pool() {
//         let f = File::create("../target/test.db");
//         if f.is_err() {
//             println!("{}", f.err().unwrap());
//         } else {
//             drop(f);
//         }
//         let f = async move {
//             let pool = Pool::new_url(SqliteDriver {}, "sqlite://../target/test.db").unwrap();
//             let mut conn = pool.get().await.unwrap();
//             conn.exec(
//                 "CREATE TABLE `biz_activity`
// (
//     `id`            TEXT PRIMARY KEY NOT NULL,
//     `name`          TEXT     DEFAULT NULL,
//     `pc_link`       TEXT     DEFAULT NULL,
//     `h5_link`       TEXT     DEFAULT NULL,
//     `sort`          TEXT     DEFAULT NULL,
//     `status`        INT      DEFAULT NULL,
//     `version`       INT      DEFAULT NULL,
//     `remark`        TEXT     DEFAULT NULL,
//     `create_time`   datetime DEFAULT NULL,
//     `delete_flag`   INT(1)   DEFAULT NULL,
//     `pc_banner_img` TEXT     DEFAULT NULL,
//     `h5_banner_img` TEXT     DEFAULT NULL
// );
//
// INSERT INTO `biz_activity`
// VALUES ('1', '活动1', NULL, NULL, '1', 1, 1, 'fff', '2019-12-12 00:00:00', 0, NULL, NULL),
//        ('178', 'test_insret', '', '', '1', 1, 0, '', '2020-06-17 20:08:13', 0, NULL, NULL),
//        ('221', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
//        ('222', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
//        ('223', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL);",
//                 vec![],
//             )
//             .await;
//
//             let data = conn
//                 .get_values("select * from biz_activity", vec![])
//                 .await
//                 .unwrap();
//             for mut x in data {
//                 println!("row: {}", x);
//             }
//         };
//         block_on!(f);
//     }
//
//     #[test]
//     fn test_sqlite_param() {
//         let task = async move {
//             let mut d = SqliteDriver {};
//             let mut c = d.connect("sqlite://../target/test.db").await.unwrap();
//             let param = vec![
//                 Decimal("1".to_string()).into(),
//                 Value::String("1".to_string()),
//             ];
//             println!("param => {}", Value::Array(param.clone()));
//             let data = c
//                 .exec("update biz_activity set version = ? where id  = ?", param)
//                 .await
//                 .unwrap();
//             println!("{}", data);
//         };
//         block_on!(task);
//     }
// }
