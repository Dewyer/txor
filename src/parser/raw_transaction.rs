use serde::{Serialize, Deserialize};
use crate::errors::ParserError;
use crate::models::{ClientId, Transaction, TransactionId, MoneyCents, DepositData, WithdrawalData, DisputeData, ResolutionData, ChargebackData};

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

fn parse_positive_amount(amount: Option<f64>) -> Result<MoneyCents, ParserError> {
    let converted_amount = (amount.ok_or(ParserError::TransactionMissingAmount)? * MONEY_PRECISION) as i64;

    if converted_amount < 0 {
        Err(ParserError::NegativeTransactionAmount)
    } else {
        Ok(converted_amount)
    }
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = ParserError;

    fn try_from(value: RawTransaction) -> Result<Self, Self::Error> {
        let referential_data = ResolutionData {
            client_id: ClientId::new(value.client_id),
            referenced_transaction_id: TransactionId::new(value.transaction_id),
        };

        match value.type_str.as_str() {
            "deposit" => Ok(Transaction::Deposit(DepositData {
                client_id: ClientId::new(value.client_id),
                transaction_id: TransactionId::new(value.transaction_id),
                amount: parse_positive_amount(value.amount)?,
            })),
            "withdrawal" => Ok(Transaction::Withdrawal(WithdrawalData {
                client_id: ClientId::new(value.client_id),
                transaction_id: TransactionId::new(value.transaction_id),
                amount: parse_positive_amount(value.amount)?,
            })),
            "dispute" => Ok(Transaction::Dispute(referential_data)),
            "resolve" => Ok(Transaction::Resolution(referential_data)),
            "chargeback" => Ok(Transaction::Chargeback(referential_data)),
            _ => Err(Self::Error::UnknownTransactionType),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ClientId, MoneyCents, Transaction, TransactionId};
    use crate::parser::raw_transaction::{MONEY_PRECISION, RawTransaction};

    #[test]
    fn simple_deposit_parsable() {
        let tx = Transaction::try_from(RawTransaction {
            type_str: "deposit".to_string(),
            client_id: 1,
            transaction_id: 1,
            amount: Some(100f64),
        }).unwrap();

        if let Transaction::Deposit(dps_tx) = tx {
            assert_eq!(dps_tx.client_id, ClientId::new(1));
            assert_eq!(dps_tx.transaction_id, TransactionId::new(1));
            assert_eq!(dps_tx.amount, (100f64 * MONEY_PRECISION) as MoneyCents);
        } else {
            panic!("expected a deposit transaction");
        }
    }

    #[test]
    fn simple_dispute_parsable() {
        let tx = Transaction::try_from(RawTransaction {
            type_str: "dispute".to_string(),
            client_id: 1,
            transaction_id: 1,
            amount: None,
        }).unwrap();

        if let Transaction::Dispute(dis_tx) = tx {
            assert_eq!(dis_tx.client_id, ClientId::new(1));
            assert_eq!(dis_tx.referenced_transaction_id, TransactionId::new(1));
        } else {
            panic!("expected a dispute transaction");
        }
    }

    #[test]
    fn negative_deposit_impossible() {
        assert!(Transaction::try_from(RawTransaction {
            type_str: "deposit".to_string(),
            client_id: 1,
            transaction_id: 1,
            amount: Some(-100f64),
        }).is_err());
    }
}