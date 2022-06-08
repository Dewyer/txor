use std::pin::Pin;
use tokio_stream::Stream;
use crate::models::{ Transaction };
use crate::errors::{ TxorError };

pub type TransactionStream = Pin<Box<dyn Stream<Item=Result<Transaction, TxorError>>>>;

pub trait TransactionSource {
    fn stream_transactions(self) -> TransactionStream;
}
