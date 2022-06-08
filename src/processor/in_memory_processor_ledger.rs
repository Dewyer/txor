use std::collections::HashMap;
use crate::models::{ClientAccount, ClientId, StoredTransaction, TransactionId};
use crate::processor::processor_ledger::ProcessorLedger;

pub struct InMemoryProcessorLedger {
    clients: HashMap<ClientId, ClientAccount>,
    processed_transaction: HashMap<TransactionId, StoredTransaction>,
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
    fn get_or_create_client(&mut self, client_id: ClientId) -> &mut ClientAccount {
        self.clients.entry(client_id)
            .or_insert(ClientAccount::new(client_id))
    }

    fn store_transaction(&mut self, transaction_id: TransactionId, transaction: StoredTransaction) {
        self.processed_transaction.insert(transaction_id, transaction);
    }

    fn into_client_accounts(mut self) -> Vec<ClientAccount> {
        self.clients.drain().map(|(_, client)| client).collect()
    }
}