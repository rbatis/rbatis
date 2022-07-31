use rbdc::Error;
use rbs::Value;

pub mod driver;
pub mod decode;
pub mod encode;

pub struct OracleConnection {
    sender: flume::Sender<Value>,
    receiver: flume::Receiver<Value>,
}

impl OracleConnection{
    pub async fn establish()->Result<Self,Error>{
       todo!()
    }
}


#[cfg(test)]
mod tests {
    use rbdc::block_on;
    use rbdc::rt::tokio;
    use rbs::Value;
    use crate::OracleConnection;

    #[test]
    fn test_oracle_pool() {
        let f = async move {
            use oracle::{Connection, Error};
            // Connect to a database.
            let conn = Connection::connect("scott", "tiger", "//localhost/XE").unwrap();
            let sql = "select ename, sal, comm from emp where deptno = :1";

            let (s, r) = flume::unbounded();
            let s_clone = s.clone();
            let r_clone = r.clone();
            std::thread::spawn(move || {
                loop {
                    if let Ok(v) = r_clone.recv() {
                        s_clone.send(v);
                    }
                }
            });
            let conn = OracleConnection {
                sender: s,
                receiver: r,
            };
            conn.sender.send_async(Value::Null).await;
        };
        block_on!(f);
    }
}
