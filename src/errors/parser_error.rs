use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("deposit and withdraw transactions must have an amount associated with them")]
    TransactionMissingAmount,
    #[error("unknown transaction type")]
    UnknownTransactionType,
    #[error("received a negative transaction amount in a transaction where its not allowed")]
    NegativeTransactionAmount,
    #[error("csv parsing error occured: `{0}`")]
    CsvError(#[from] csv_async::Error),
}
