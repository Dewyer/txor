use crate::models::{ Transaction };
use super::raw_transaction::RawTransaction;
use async_stream::stream;
use crate::errors::{ParserError, TxorError};
use tokio::io;
use futures::stream::StreamExt;
use crate::parser::{TransactionSource, TransactionStream};

pub struct CsvTransactionSource {
    reader: Box<dyn io::AsyncRead + Send + Unpin>,
}

impl CsvTransactionSource {
    pub fn from_reader(reader: impl io::AsyncRead  + 'static + Send + Unpin) -> Self {
        Self {
            reader: Box::new(reader),
        }
    }
}

impl TransactionSource for CsvTransactionSource {
    fn stream_transactions(self) -> TransactionStream {
        Box::pin(stream! {
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
        })
    }
}