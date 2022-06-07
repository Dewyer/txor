use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("unknown transaction type")]
    UnknownTransactionType,
}