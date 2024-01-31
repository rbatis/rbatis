use crate::Error;
use futures_core::future::BoxFuture;
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor};

pub trait Tx {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>>
    where
        Self: Sized;
    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>>;
    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>>;
}

impl Tx for RBatisTxExecutor {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.exec("begin", vec![]).await?;
            Ok(self)
        })
    }

    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("rollback", vec![]).await?;
            Ok(())
        })
    }

    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("commit", vec![]).await?;
            Ok(())
        })
    }
}


impl Tx for RBatisConnExecutor {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.exec("begin", vec![]).await?;
            Ok(self)
        })
    }

    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("rollback", vec![]).await?;
            Ok(())
        })
    }

    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("commit", vec![]).await?;
            Ok(())
        })
    }
}