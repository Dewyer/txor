use crate::models::{DepositData, DisputeState};

#[derive(Clone)]
pub struct StoredTransaction {
    data: DepositData,
    dispute_state: DisputeState,
}

impl StoredTransaction {
    pub fn new(data: DepositData) -> Self {
        Self {
            data,
            dispute_state: DisputeState::Undisputed,
        }
    }

    pub fn get_data(&self) -> &DepositData {
        &self.data
    }

    pub fn get_dispute_state(&self) -> &DisputeState {
        &self.dispute_state
    }

    pub fn dispute(&mut self) {
        self.dispute_state = DisputeState::Disputed;
    }

    pub fn remove_dispute(&mut self) {
        self.dispute_state = DisputeState::Undisputed;
    }
}