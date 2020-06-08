use log::{error, info, warn};
use rbatis_core::mysql::{MySqlPool, MySqlRow, MySqlCursor};
use rbatis_core::types::BigDecimal;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use crate::example::conf::MYSQL_URL;
use rbatis_core::executor::Executor;
use rbatis_core::cursor::Cursor;


#[test]
pub fn test_log() {
    //1 启用日志(可选，不添加则不加载日志库)
    fast_log::log::init_log("requests.log").unwrap();
    info!("print data");
    sleep(Duration::from_secs(1));
}




#[test]
pub fn test_mysql_driver() {
    async_std::task::block_on(
        async move {
            let pool = MySqlPool::new(MYSQL_URL).await.unwrap();
            //pooledConn 交由rbatis上下文管理
            let mut start = SystemTime::now();
            let mut conn = pool.acquire().await.unwrap();
            start = SystemTime::now();
            let mut c = conn.fetch("SELECT count(1) FROM biz_activity;");
            start = SystemTime::now();
            let r:BigDecimal = c.decode().await.unwrap();
            println!("done:{:?}",r);
        }
    );
}

