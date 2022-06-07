use crate::models::{ClientId, TransactionId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    Deposit {
        client_id: ClientId,
        transaction_id: TransactionId,
        amount: f64,
    },
    Withdraw {
        client_id: ClientId,
        transaction_id: TransactionId,
        amount: f64,
    },
}