use crate::models::{ClientId, TransactionId};
use serde::{Serialize, Deserialize};
use crate::models::money_cents::MoneyCents;

#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    Deposit {
        client_id: ClientId,
        transaction_id: TransactionId,
        amount: MoneyCents,
    },
    Withdraw {
        client_id: ClientId,
        transaction_id: TransactionId,
        amount: MoneyCents,
    },
}