use crate::connection::PgConnection;
use crate::options::PgConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver, Placeholder};
use rbdc::Error;

#[derive(Debug)]
pub struct PgDriver {}

impl Driver for PgDriver {
    fn name(&self) -> &str {
        "postgres"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let conn = PgConnection::establish(&url.parse()?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }
    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt = opt.downcast_ref().unwrap();
        Box::pin(async move {
            let conn = PgConnection::establish(opt).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }
    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(PgConnectOptions::default())
    }
}

impl Placeholder for PgDriver {
    fn exchange(&self, sql: &str) -> String {
        let mut last = ' ' as u8;
        let mut sql_bytes = sql.as_bytes().to_vec();
        let mut placeholder_idx: i32 = 1;
        let mut index = 0;
        loop {
            if index == sql_bytes.len() {
                break;
            }
            let x = sql_bytes[index];
            if x == '?' as u8 && last != '\\' as u8 {
                sql_bytes[index] = '$' as u8;
                let bytes = placeholder_idx.to_string().into_bytes();
                let mut idx = 0;
                for x in bytes {
                    sql_bytes.insert(index + 1 + idx, x);
                    last = x;
                    idx += 1;
                }
                placeholder_idx += 1;
            } else {
                last = x;
            }
            index += 1;
        }
        String::from_utf8(sql_bytes).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use rbdc::db::Placeholder;
    use crate::driver::PgDriver;

    #[test]
    fn test_exchange() {
        let v = "insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)";
        let d = PgDriver {};
        let sql = d.exchange(v);
        assert_eq!("insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)", sql);
    }
}

// #[cfg(test)]
// mod test {
//     use crate::driver::PgDriver;
//     use rbdc::block_on;
//     use rbdc::db::Driver;
//     use rbdc::db::Placeholder;
//     use rbdc::decimal::Decimal;
//     use rbdc::pool::Pool;
//     use rbdc::timestamp::Timestamp;
//     use rbs::Value;
//
//     #[test]
//     fn test_pg_pool() {
//         let task = async move {
//             let pool = Pool::new_url(
//                 PgDriver {},
//                 "postgres://postgres:123456@localhost:5432/postgres",
//             )
//             .unwrap();
//             std::thread::sleep(std::time::Duration::from_secs(2));
//             let mut conn = pool.get().await.unwrap();
//             let data = conn
//                 .get_values("select * from biz_activity", vec![])
//                 .await
//                 .unwrap();
//             for mut x in data {
//                 println!("row: {}", x);
//             }
//         };
//         block_on!(task);
//     }
//
//     #[test]
//     fn test_pg_param() {
//         let task = async move {
//             let mut d = PgDriver {};
//             let mut c = d
//                 .connect("postgres://postgres:123456@localhost:5432/postgres")
//                 .await
//                 .unwrap();
//             let param = vec![
//                 Value::String("http://www.test.com".to_string()),
//                 Timestamp(1659996552000).into(),
//                 Decimal("1".to_string()).into(),
//                 Value::String("1".to_string()),
//             ];
//             println!("param => {}", Value::Array(param.clone()));
//             let data = c
//                 .exec(
//                     "update biz_activity set pc_link = $1,create_time = $2,delete_flag=$3 where id  = $4",
//                     param,
//                 )
//                 .await
//                 .unwrap();
//             println!("{}", data);
//         };
//         block_on!(task);
//     }
//
//     #[test]
//     fn test_exchange() {
//         let d = PgDriver {};
//         let s = d.exchange("select * from table where id = ? age = ?");
//         println!("{}", s);
//         assert_eq!(s, "select * from table where id = $1 age = $2")
//     }
// }
