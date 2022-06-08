use crate::models::{ClientId, TransactionId};
use serde::{Serialize, Deserialize};
use crate::models::money_cents::MoneyCents;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct AssetMovementData {
    pub client_id: ClientId,
    pub transaction_id: TransactionId,
    pub amount: MoneyCents,
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
pub struct ReferentialData {
    pub client_id: ClientId,
    pub referenced_transaction_id: TransactionId,
}

pub type DepositData = AssetMovementData;
pub type WithdrawalData = AssetMovementData;

pub type DisputeData = ReferentialData;
pub type ResolutionData = ReferentialData;
pub type ChargebackData = ReferentialData;

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub enum Transaction {
    Deposit(DepositData),
    Withdrawal(WithdrawalData),
    Dispute(DisputeData),
    Resolution(ResolutionData),
    Chargeback(ChargebackData),
}