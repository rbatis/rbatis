use rbdc::Error;
use rbs::Value;

pub mod driver;
pub mod decode;
pub mod encode;

pub struct OracleConnection {
    sender_arg: flume::Sender<(String, Vec<Value>)>,
    receiver_arg: flume::Receiver<(String, Vec<Value>)>,
    sender_result: flume::Sender<Result<Value, Error>>,
    receiver_result: flume::Receiver<Result<Value, Error>>,
}

impl OracleConnection {
    pub async fn establish() -> Result<Self, Error> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use flume::SendError;
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
            let (s_r, r_r) = flume::unbounded();
            let conn = OracleConnection {
                sender_arg: s,
                receiver_arg: r,
                sender_result: s_r,
                receiver_result: r_r,
            };
            let s = conn.sender_result.clone();
            let r = conn.receiver_arg.clone();
            std::thread::spawn(move || {
                loop {
                    if let Ok(v) = r.recv() {
                        let result = Ok(Value::Null);
                        if let Err(_) = s.send(result) {
                            //disconnected exit thread
                            break;
                        }
                    } else {
                        //disconnected exit thread
                        break;
                    }
                }
            });
            conn.sender_arg.send_async(("select * from table".to_string(), vec![])).await;
        };
        block_on!(f);
    }
}
