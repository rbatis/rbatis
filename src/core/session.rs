use serde::de;
use serde_json::Value;

///事务传播行为
pub enum Propagation {
    ///默认，表示如果当前事务存在，则支持当前事务。否则，会启动一个新的事务。have tx ? join : new tx()
    REQUIRED,
    ///表示如果当前事务存在，则支持当前事务，如果当前没有事务，就以非事务方式执行。  have tx ? join(): session.exec()
    SUPPORTS,
    ///表示如果当前事务存在，则支持当前事务，如果当前没有事务，则返回事务嵌套错误。  have tx ? join() : return error
    MANDATORY,
    ///表示新建一个全新Session开启一个全新事务，如果当前存在事务，则把当前事务挂起。 have tx ? stop old。  -> new session().new tx()
    REQUIRES_NEW,
    ///表示以非事务方式执行操作，如果当前存在事务，则新建一个Session以非事务方式执行操作，把当前事务挂起。  have tx ? stop old。 -> new session().exec()
    NOT_SUPPORTED,
    ///表示以非事务方式执行操作，如果当前存在事务，则返回事务嵌套错误。    have tx ? return error: session.exec()
    NEVER,
    ///表示如果当前事务存在，则在嵌套事务内执行，如嵌套事务回滚，则只会在嵌套事务内回滚，不会影响当前事务。如果当前没有事务，则进行与PROPAGATION_REQUIRED类似的操作。
    NESTED,
    ///表示如果当前没有事务，就新建一个事务,否则返回错误。  have tx ? return error: session.new tx()
    NOT_REQUIRED,
}


pub trait Session {
    fn id(&self) -> String;
    fn query<T>(&mut self,sql: &str, arg_array: &mut Vec<Value>) -> Result<T, String> where T: de::DeserializeOwned;
    fn exec(&mut self,sql: &str, arg_array: &mut Vec<Value>) -> Result<u64, String>;

    fn rollback(&mut self) -> Result<u64, String>;
    fn commit(&mut self) -> Result<u64, String>;
    fn begin(&mut self,propagation_type: Propagation) -> Result<u64, String>;
    fn close(&mut self,);
    fn propagation(&self) -> Propagation;
}