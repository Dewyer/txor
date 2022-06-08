use std::collections::HashMap;
use crate::models::{ClientAccount, ClientId, Transaction, TransactionId};
use crate::processor::processor_ledger::ProcessorLedger;

pub struct InMemoryProcessorLedger {
    clients: HashMap<ClientId, ClientAccount>,
    processed_transaction: HashMap<TransactionId, Transaction>,
}

impl InMemoryProcessorLedger {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            processed_transaction: HashMap::new(),
        }
    }
}

impl ProcessorLedger for InMemoryProcessorLedger {

}