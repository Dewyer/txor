use std::collections::HashMap;
use crate::errors::ProcessorError;
use crate::models::{ClientAccount, ClientId, DisputeState, StoredTransaction, TransactionId};
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

    fn has_stored_transaction(&self,  transaction_id: TransactionId) -> bool {
        self.processed_transaction.contains_key(&transaction_id)
    }

    fn get_stored_transaction_mut(&mut self, transaction_id: TransactionId) -> Option<&mut StoredTransaction> {
        self.processed_transaction.get_mut(&transaction_id)
    }

    fn get_stored_transaction(&self, transaction_id: TransactionId) -> Option<&StoredTransaction> {
        self.processed_transaction.get(&transaction_id)
    }

    fn into_client_accounts(mut self) -> Vec<ClientAccount> {
        self.clients.drain().map(|(_, client)| client).collect()
    }

    fn get_transactions_in_dispute(&self) -> Vec<TransactionId> {
        self.processed_transaction.values()
            .filter(|tx| tx.get_dispute_state() == &DisputeState::Disputed)
            .map(|tx| tx.get_data().transaction_id)
            .collect()
    }
}