use tokio_stream::Stream;
use crate::models::{ Transaction };
use super::raw_transaction::RawTransaction;
use async_stream::stream;
use crate::errors::{ParserError, TxorError};
use tokio::io;
use futures::stream::StreamExt;

pub struct TransactionSource {
    reader: Box<dyn io::AsyncRead + Send + Unpin>,
}

impl TransactionSource {
    pub fn from_reader(reader: impl io::AsyncRead  + 'static + Send + Unpin) -> Self {
        Self {
            reader: Box::new(reader),
        }
    }

    pub fn stream_transactions(self) -> impl Stream<Item=Result<Transaction, TxorError>> {
        stream! {
            let mut rdr = csv_async::AsyncReaderBuilder::new()
                .trim(csv_async::Trim::All)
                .create_deserializer(self.reader);
            let mut records = rdr.deserialize::<RawTransaction>();

            while let Some(result) = records.next().await {
                yield result
                    .map_err(|err| TxorError::Parser(ParserError::CsvError(err)))
                    .and_then(|raw_tx|
                        Transaction::try_from(raw_tx)
                            .map_err(|err| TxorError::from(err))
                    );
            }
        }
    }
}