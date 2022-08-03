use async_trait::async_trait;
use futures_core::future::BoxFuture;
use rbatis_core::Error;
use rbdc::db::Connection;

#[async_trait]
pub trait Tx{
    async fn begin(mut self) -> Result<Self,Error> where Self:Sized;
    async fn rollback(&mut self) -> Result<(),Error>;
    async fn commit(&mut self) -> Result<(),Error>;
}

#[async_trait]
impl Tx for Box<dyn Connection>{
    async fn begin(self) -> Result<Self, Error> {
        todo!()
    }

    async fn rollback(&mut self) -> Result<(), Error> {
        todo!()
    }

    async fn commit(&mut self) -> Result<(), Error> {
        todo!()
    }
}