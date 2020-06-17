use std::sync::Arc;
use std::cell::RefCell;
use rbatis_core::pool::PoolConnection;
use rbatis_core::mysql::MySqlConnection;

//TODO impl Context hold tx
pub struct Context{
    //TODO Arc use async std's Arc ???
    tx:Arc<RefCell<Option<PoolConnection<MySqlConnection>>>>,
}
