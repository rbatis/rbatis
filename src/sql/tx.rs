use async_trait::async_trait;
use futures_core::future::BoxFuture;
use rbatis_core::Error;
use rbdc::db::Connection;

#[async_trait]
pub trait Tx {
    async fn begin(mut self) -> Result<Self, Error>
    where
        Self: Sized;
    async fn rollback(&mut self) -> Result<(), Error>;
    async fn commit(&mut self) -> Result<(), Error>;
}

#[async_trait]
impl Tx for Box<dyn Connection> {
    async fn begin(mut self) -> Result<Self, Error> {
        self.exec("begin", vec![]).await?;
        Ok(self)
    }

    async fn rollback(&mut self) -> Result<(), Error> {
        self.exec("rollback", vec![]).await?;
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), Error> {
        self.exec("commit", vec![]).await?;
        Ok(())
    }
}
