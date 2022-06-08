use thiserror::Error;
use crate::models::{ClientId, MoneyCents, TransactionId};

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("can't perform transaction with locked client {0}")]
    ClientLocked(ClientId),
    #[error("client {0} doesn't has insufficient funds to perform transaction {1}, required: {2}")]
    InsufficientFunds(ClientId, TransactionId, MoneyCents),
}
