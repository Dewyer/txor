use serde::{Serialize, Deserialize};
use crate::errors::ParserError;
use crate::models::{ClientId, Transaction, TransactionId};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawTransaction {
    #[serde(rename(deserialize = "type"))]
    pub type_str: String,

    #[serde(rename(deserialize = "client"))]
    pub client_id: u16,

    #[serde(rename(deserialize = "tx"))]
    pub transaction_id: u32,

    pub amount: Option<f64>,
}

const MONEY_PRECISION: f64 = 1000f64;

fn parse_amount(amount: Option<f64>) -> Result<i64, ParserError> {
    Ok((amount.ok_or(ParserError::TransactionMissingAmount)? * MONEY_PRECISION) as i64)
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = ParserError;

    fn try_from(value: RawTransaction) -> Result<Self, Self::Error> {
        match value.type_str.as_str() {
            "deposit" => Ok(Transaction::Deposit {
                client_id: ClientId::new(value.client_id),
                transaction_id: TransactionId::new(value.transaction_id),
                amount: parse_amount(value.amount)?,
            }),
            "withdrawal" => Ok(Transaction::Withdraw {
                client_id: ClientId::new(value.client_id),
                transaction_id: TransactionId::new(value.transaction_id),
                amount: parse_amount(value.amount)?,
            }),
            _ => Err(Self::Error::UnknownTransactionType),
        }
    }
}