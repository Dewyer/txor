use crate::models::DepositData;

pub struct StoredTransaction {
    data: DepositData,
    disputed: bool,
}

impl StoredTransaction {
    pub fn new(data: DepositData) -> Self {
        Self {
            data,
            disputed: false,
        }
    }
}