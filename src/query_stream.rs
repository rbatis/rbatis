use crate::Error;
use futures_core::Stream;
use rbdc::rt::tokio::sync::mpsc;
use rbs::Value;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A bounded, cancellation-aware stream of database rows.
///
/// Dropping the stream closes its receiver. Native query producers observe the
/// closed channel and stop polling the database row stream.
#[derive(Debug)]
pub struct QueryStream {
    receiver: mpsc::Receiver<Result<Value, Error>>,
}

impl QueryStream {
    pub(crate) fn channel(
        prefetch: usize,
    ) -> Result<(mpsc::Sender<Result<Value, Error>>, Self), Error> {
        if prefetch == 0 {
            return Err(Error::from(
                "[rb] query stream prefetch must be greater than zero",
            ));
        }
        let (sender, receiver) = mpsc::channel(prefetch);
        Ok((sender, Self { receiver }))
    }

    pub(crate) fn from_value(value: Value, prefetch: usize) -> Result<Self, Error> {
        let (sender, stream) = Self::channel(prefetch)?;
        let rows = match value {
            Value::Array(rows) => rows,
            value => vec![value],
        };
        rbdc::rt::spawn(async move {
            for row in rows {
                if sender.send(Ok(row)).await.is_err() {
                    break;
                }
            }
        });
        Ok(stream)
    }
}

impl Stream for QueryStream {
    type Item = Result<Value, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

impl Drop for QueryStream {
    fn drop(&mut self) {
        self.receiver.close();
    }
}
