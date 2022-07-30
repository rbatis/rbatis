
use mobc::{async_trait, Manager, Pool};
use crate::block_on;
use crate::db::{ConnectOptions, Driver};

pub struct RBDCManager{
    driver:Box<dyn Driver>,
    opt:Box<dyn ConnectOptions>,
}

#[async_trait]
impl Manager for RBDCManager {
    type Connection = Box<dyn crate::db::Connection>;
    type Error = crate::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // self.driver.connect()
        todo!()
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}

#[test]
fn test_pool(){
    let f=async move{
        // let foo=RBDCManager{
        //     driver: Box::new(),
        //     opt: Box::new(())
        // };
        // let p=Pool::new(foo);
        // p.set_max_open_conns(10);
        // for _ in 0..1000{
        //     p.get().await.unwrap();
        // }
    };
    block_on!(f);
}