use std::time::SystemTime;

use log::{error, info, warn};
use rbatis_drivers::Connection;
use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::abstract_session::AbstractSession;
use crate::decode::driver_decoder::decode_result_set;
use crate::error::RbatisError;
use crate::example::conf::MYSQL_URL;
use crate::rbatis::Rbatis;
use crate::tx::propagation::Propagation;
use crate::tx::save_point_stack::SavePointStack;
use crate::tx::tx::{Tx, TxImpl};
use crate::tx::tx_stack::TxStack;
use crate::utils::{driver_util, rbatis_driver_util};
use crate::utils::rbatis_driver_util::{FormatString, to_driver_values};

pub struct LocalSession {
    pub session_id: String,
    pub driver: String,
    pub tx_stack: TxStack,
    pub save_point_stack: SavePointStack,
    pub is_closed: bool,
    pub new_local_session: Option<Box<LocalSession>>,
    pub enable_log: bool,
    pub conn: Option<Box<dyn Connection>>,
    pub check_alive_time: SystemTime
}


impl LocalSession {
    pub fn new(id: &str, driver: &str, conn_opt: Option<Box<dyn Connection>>,enable_log:bool) -> Result<Self, RbatisError> {
        let mut conn = conn_opt;
        if conn.is_none() {
            let r = driver_util::get_conn_by_link(driver)?;
            conn = Some(r);
        }
        return Ok(Self {
            session_id: id.to_string(),
            driver: driver.to_string(),
            tx_stack: TxStack::new(),
            save_point_stack: SavePointStack::new(),
            is_closed: false,
            new_local_session: None,
            enable_log: enable_log,
            conn: conn,
            check_alive_time: SystemTime::now()
        });
    }

    pub fn have_tx(&self) -> bool {
        if self.tx_stack.len() > 0 || self.new_local_session.is_some() {
            return true;
        }
        return false;
    }


    pub fn id(&self) -> &str {
        return self.session_id.as_str();
    }

    pub fn query<T>(&mut self, sql: &str, arg_array: &[rbatis_drivers::Value]) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        if self.is_closed == true {
            return Err(RbatisError::from("[rbatis] session can not query a closed session!".to_string()));
        }
        if self.new_local_session.is_some() {
            return self.new_local_session.as_mut().unwrap().query(sql, arg_array);
        }
        if self.enable_log {
            Rbatis::channel_send(format!("[{}]Query: ==>  {}", self.id(), sql));
            Rbatis::channel_send(format!("[{}]Args : ==>  {}", self.id(), arg_array.format_string()));
        }
        let (t_opt, _) = self.tx_stack.last_ref_mut();
        if t_opt.is_some() {
            let type_name = std::any::type_name::<T>();
            let t = t_opt.unwrap();
            let result = t.query(sql, arg_array, self.conn.as_mut().unwrap());
            return result;
        } else {
            return self.conn.as_mut().unwrap().query_prepare(self.enable_log, sql, &arg_array);
        }
    }

    pub fn exec(&mut self, sql: &str, arg_array: &[rbatis_drivers::Value]) -> Result<u64, RbatisError> {
        if self.is_closed == true {
            return Err(RbatisError::from("[rbatis] session can not query a closed session!".to_string()));
        }
        if self.new_local_session.is_some() {
            return self.new_local_session.as_mut().unwrap().query(sql, arg_array);
        }
        if self.enable_log {
            Rbatis::channel_send(format!("[{}]Query: ==>  {}", self.id(), sql));
            Rbatis::channel_send(format!("[{}]Args : ==>  {}", self.id(), arg_array.format_string()));
        }
        let (t_opt, _) = self.tx_stack.last_ref_mut();
        if t_opt.is_some() {
            let t = t_opt.unwrap();
            let result = t.exec(sql, arg_array, self.conn.as_mut().unwrap())?;
            return Ok(result);
        } else {
            return self.conn.as_mut().unwrap().exec_prepare(self.enable_log, sql, &arg_array);
        }
    }

    pub fn rollback(&mut self) -> Result<u64, RbatisError> {
        if self.is_closed == true {
            return Err(RbatisError::from("[rbatis] session can not query a closed session!".to_string()));
        }
        let mut closec_num = 0;
        if self.new_local_session.is_some() {
            let new_session = self.new_local_session.as_mut().unwrap();
            if self.enable_log {
                //info!(" [{}] Exec: ==>   Rollback; ", self.session_id);
                Rbatis::channel_send(format!(" [{}] Exec: ==>   Rollback; ", self.session_id));
            }
            let r = new_session.rollback()?;
            new_session.close();
            closec_num += r;
        }

        let (t_opt, p_opt) = self.tx_stack.pop();
        if t_opt.is_some() && p_opt.is_some() {
            let mut t = t_opt.unwrap();
            if p_opt.is_some() {
                if p_opt.as_ref().unwrap().eq(&Propagation::NESTED) {
                    let point_opt = self.save_point_stack.pop();
                    if point_opt.is_some() {
                        let sql = "rollback to ".to_string() + point_opt.unwrap().as_str();
                        let r = t.exec(sql.as_str(), &mut vec![], self.conn.as_mut().unwrap())?;
                        closec_num += r;
                    }
                }
            }
            if self.tx_stack.len() == 0 {
                if self.enable_log {
                    info!(" [{}] Exec: ==>   Rollback; ", self.id());
                }
                let r = t.rollback(self.conn.as_mut().unwrap())?;
                closec_num += r;
            }
        }
        return Ok(closec_num);
    }

    pub fn commit(&mut self) -> Result<u64, RbatisError> {
        if self.is_closed == true {
            return Err(RbatisError::from("[rbatis] session can not query a closed session!".to_string()));
        }
        let mut closec_num = 0;
        if self.new_local_session.is_some() {
            let new_session = self.new_local_session.as_mut().unwrap();
            if self.enable_log {
                //info!(" [{}] Exec: ==>   Commit; ", self.session_id);
                Rbatis::channel_send(format!(" [{}] Exec: ==>   Commit; ", self.session_id));
            }
            let r = new_session.commit()?;
            new_session.close();
            closec_num += r;
        }
        let (t_opt, p_opt) = self.tx_stack.pop();
        if t_opt.is_some() && p_opt.is_some() {
            let mut t = t_opt.unwrap();
            if p_opt.is_some() {
                if p_opt.as_ref().unwrap().eq(&Propagation::NESTED) {
                    let p_id = format!("p{}", self.tx_stack.len() + 1);
                    self.save_point_stack.push(p_id.as_str());
                    let sql = format!("savepoint {}", p_id.as_str());
                    let r = t.exec(sql.as_str(), &mut vec![], self.conn.as_mut().unwrap())?;
                    closec_num += r;
                }
            }
            if self.tx_stack.len() == 0 {
                if self.enable_log {
                    info!(" [{}] Exec: ==>   Commit; ", self.id());
                }
                let r = t.commit(self.conn.as_mut().unwrap())?;
                closec_num += r;
            }
        }
        return Ok(closec_num);
    }

    pub fn begin(&mut self, id:&str,propagation_type: Propagation) -> Result<u64, RbatisError> {
        if self.is_closed == true {
            return Err(RbatisError::from("[rbatis] session can not query a closed session!".to_string()));
        }
        if self.enable_log {
            //info!(" [{}] Exec: ==>   Begin:{}; ", self.id(), propagation_type);
            Rbatis::channel_send(format!(" [{}] Exec: ==>   Begin:{}; ", self.id(), propagation_type));
        }
        match propagation_type {
            //默认，表示如果当前事务存在，则支持当前事务。否则，会启动一个新的事务。have tx ? join : new tx()
            Propagation::REQUIRED => {
                let tx = TxImpl::begin(id, self.driver.as_str(), self.enable_log, self.conn.as_mut().unwrap())?;
                self.tx_stack.push(tx, Propagation::REQUIRED);
            }
            //表示如果当前事务存在，则支持当前事务，如果当前没有事务，就以非事务方式执行。  have tx ? join(): session.exec()
            Propagation::SUPPORTS => {
                if self.tx_stack.len() > 0 {
                    let tx = TxImpl::begin(id, self.driver.as_str(), self.enable_log, self.conn.as_mut().unwrap())?;
                    self.tx_stack.push(tx, Propagation::SUPPORTS);
                } else {}
                return Ok(0);
            }
            //表示如果当前事务存在，则支持当前事务，如果当前没有事务，则返回事务嵌套错误。  have tx ? join() : return error
            Propagation::MANDATORY => {
                if self.tx_stack.len() > 0 {
                    let tx = TxImpl::begin(id, self.driver.as_str(), self.enable_log, self.conn.as_mut().unwrap())?;
                    self.tx_stack.push(tx, Propagation::MANDATORY);
                    return Ok(0);
                } else {
                    return Err(RbatisError::from("[rbatis] PROPAGATION_MANDATORY Nested transaction exception! current not have a transaction!".to_string()));
                }
            }
            //表示新建一个全新Session开启一个全新事务，如果当前存在事务，则把当前事务挂起。 have tx ? stop old。  -> new session().new tx()
            Propagation::REQUIRES_NEW => {
                //new session
                let r = driver_util::get_conn_by_link(self.driver.as_str())?;
                let new_session = LocalSession::new("", self.driver.as_str(), Option::from(r),self.enable_log)?;
                self.new_local_session = Some(Box::new(new_session));
            }
            //表示以非事务方式执行操作，如果当前存在事务，则新建一个Session以非事务方式执行操作，把当前事务挂起。  have tx ? stop old。 -> new session().exec()
            Propagation::NOT_SUPPORTED => {
                if self.tx_stack.len() > 0 {
                    let r = driver_util::get_conn_by_link(self.driver.as_str())?;
                    let new_session = LocalSession::new("", self.driver.as_str(), Option::from(r),self.enable_log)?;
                    self.new_local_session = Some(Box::new(new_session));
                }
            }
            //表示以非事务方式执行操作，如果当前存在事务，则返回事务嵌套错误。    have tx ? return error: session.exec()
            Propagation::NEVER => {
                if self.tx_stack.len() > 0 {
                    return Err(RbatisError::from("[rbatis] PROPAGATION_NEVER  Nested transaction exception! current Already have a transaction!".to_string()));
                }
            }
            //表示如果当前事务存在，则在嵌套事务内执行，如嵌套事务回滚，则只会在嵌套事务内回滚，不会影响当前事务。如果当前没有事务，则进行与PROPAGATION_REQUIRED类似的操作。
            Propagation::NESTED => {
                if self.tx_stack.len() > 0 {
                    let tx = TxImpl::begin(id, self.driver.as_str(), self.enable_log, self.conn.as_mut().unwrap())?;
                    self.tx_stack.push(tx, Propagation::NESTED);
                } else {
                    return self.begin(id,Propagation::REQUIRED);
                }
            }
            //表示如果当前没有事务，就新建一个事务,否则返回错误。  have tx ? return error: session.new tx()
            Propagation::NOT_REQUIRED => {
                if self.tx_stack.len() > 0 {
                    return Err(RbatisError::from("[rbatis] PROPAGATION_NOT_REQUIRED Nested transaction exception! current Already have a transaction!".to_string()));
                } else {
                    //new tx
                    let tx = TxImpl::begin(id, self.driver.as_str(), self.enable_log, self.conn.as_mut().unwrap())?;
                    self.tx_stack.push(tx, propagation_type);
                }
            }
            Propagation::NONE => {
                return Ok(0);
            }
            _ => {
                return Err(RbatisError::from("[rbatis] Nested transaction exception! not support PROPAGATION in begin!".to_string()));
            }
        }
        return Ok(0);
    }

    pub fn is_valid(&mut self) -> bool {
        return self.conn.as_mut().unwrap().is_valid();
    }

    pub fn close(&mut self) {
        if self.is_closed {
            return;
        }
        self.tx_stack.close();
        self.is_closed = true;
    }
}

impl Drop for LocalSession {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
pub fn test_se() {
    let s = LocalSession::new("", MYSQL_URL, None,true);
    if s.is_err() {
        println!("执行失败:{}", s.err().unwrap());
        return;
    }
    let mut se = s.unwrap();
    se.begin("",Propagation::NONE);
}
