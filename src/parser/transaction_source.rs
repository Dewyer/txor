use crate::errors::TxorError;
use crate::models::Transaction;
use std::pin::Pin;
use tokio_stream::Stream;

pub type TransactionStream = Pin<Box<dyn Stream<Item = Result<Transaction, TxorError>>>>;

pub trait TransactionSource {
    fn stream_transactions(self) -> TransactionStream;
}
