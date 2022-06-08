use thiserror::Error;
use crate::models::{ClientId, MoneyCents, TransactionId};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("can't perform transaction with locked client {0}")]
    ClientLocked(ClientId),
    #[error("client {0} doesn't has insufficient funds to perform transaction {1}, required: {2}")]
    InsufficientFunds(ClientId, TransactionId, MoneyCents),
    #[error("transaction by this id(`{0}`) already exists")]
    TransactionAlreadyExists(TransactionId),
    #[error("transaction by this id(`{0}`) doesn't exists or can't be disputed")]
    TransactionDoesntExists(TransactionId),
    #[error("transaction by this id(`{0}`) is already being disputed")]
    TransactionAlreadyDisputed(TransactionId),
    #[error("transaction by this id(`{0}`) is not being disputed")]
    TransactionNotDisputed(TransactionId),
    #[error("client `{0}` doesnt have access to transaction `{}`")]
    ClientInsufficientAccess(ClientId, TransactionId),
}
