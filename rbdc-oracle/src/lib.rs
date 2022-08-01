use std::str::FromStr;
use std::sync::Arc;
use oracle::Connection;
use rbdc::Error;
use rbs::Value;
use serde::{Serialize, Deserialize};

pub mod driver;
pub mod decode;
pub mod encode;

#[derive(Clone)]
pub struct OracleConnection {
    sender_arg: flume::Sender<(String, Vec<Value>)>,
    receiver_arg: flume::Receiver<(String, Vec<Value>)>,
    sender_result: flume::Sender<Result<Value, Error>>,
    receiver_result: flume::Receiver<Result<Value, Error>>,
    pub conn: Arc<Connection>,
}

impl OracleConnection {
    /// connect : thread will be 1:1 Relationship
    pub async fn establish(opt: OracleConnectOptions) -> Result<Self, Error> {
        let conn = Connection::connect(opt.username, opt.password, opt.connect_string).map_err(|e| Error::from(e.to_string()))?;
        let (s,r)=flume::unbounded();
        let (sender_result,recv_result)=flume::unbounded();
        let conn = OracleConnection {
            sender_arg: s,
            receiver_arg: r,
            sender_result: sender_result,
            receiver_result: recv_result,
            conn: Arc::new(conn),
        };
        let conn_clone = conn.clone();
        std::thread::spawn(move || {
            loop {
                if let Ok((sql, args)) = conn_clone.receiver_arg.recv() {
                    let result = conn_clone.do_command(sql, args);
                    if let Err(_) = conn_clone.sender_result.send(result) {
                        //disconnected exit thread
                        break;
                    }
                } else {
                    //disconnected exit thread
                    break;
                }
            }
        });
        Ok(conn)
    }

    pub fn do_command(&self, sql: String, args: Vec<Value>) -> Result<Value,Error> {
        //todo params impl args
        if sql.starts_with("select"){
            let q=self.conn.query(&sql, &[]).map_err(|e| Error::from(e.to_string()))?;
            let data = vec![];
            for x in q {
                let row=x.map_err(|e| Error::from(e.to_string()))?;
                todo!()
            }
            Ok(Value::Array(data))
        }else{
            let v=self.conn.execute(&sql, &[]).map_err(|e| Error::from(e.to_string()))?;
            Ok(Value::U64(v.row_count().unwrap_or(0)))
        }
    }
}

///Connects to an Oracle server using username, password and connect string.
/// If you need to connect the server with additional parameters such as SYSDBA privilege, use Connector instead.
/// Examples
/// Connect to a local database.
/// let conn = Connection::connect("scott", "tiger", "")?;
//
/// Connect to a remote database specified by easy connect naming.
/// let conn = Connection::connect("scott", "tiger",
///                                "server_name:1521/service_name")?;
#[derive(Serialize, Deserialize)]
pub struct OracleConnectOptions {
    pub username: String,
    pub password: String,
    pub connect_string: String,
}

impl FromStr for OracleConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| Error::from(e.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use flume::SendError;
    use rbdc::block_on;
    use rbdc::rt::tokio;
    use rbs::Value;
    use crate::{OracleConnection, OracleConnectOptions};

    #[test]
    fn test_oracle_pool() {
        let f = async move {
            let sql = "select ename, sal, comm from emp where deptno = :1";
            let conn = OracleConnection::establish(OracleConnectOptions {
                username: "".to_string(),
                password: "".to_string(),
                connect_string: "//localhost/XE".to_string(),
            }).await.unwrap();
        };
        block_on!(f);
    }
}
