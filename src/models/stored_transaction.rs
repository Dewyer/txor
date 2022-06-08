use crate::models::DepositData;

#[derive(Clone)]
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

    pub fn get_data(&self) -> &DepositData {
        &self.data
    }

    pub fn is_disputed(&self) -> bool {
        self.disputed
    }

    pub fn dispute(&mut self) {
        self.disputed = true;
    }

    pub fn remove_dispute(&mut self) {
        self.disputed = false;
    }
}