use crate::models::{ClientAccount, TransactionId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProcessingOutput {
    pub clients: Vec<ClientAccount>,

    pub transactions_in_dispute: Vec<TransactionId>,
}
