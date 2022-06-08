use crate::models::{ClientAccount, ClientId, StoredTransaction, TransactionId};

pub trait ProcessorLedger {
    fn get_or_create_client(&mut self, client_id: ClientId) -> &mut ClientAccount;

    fn store_transaction(&mut self, transaction_id: TransactionId, transaction: StoredTransaction);

    fn has_stored_transaction(&self, transaction_id: TransactionId) -> bool;

    fn get_stored_transaction_mut(&mut self, transaction_id: TransactionId) -> Option<&mut StoredTransaction>;

    fn get_stored_transaction(&self, transaction_id: TransactionId) -> Option<&StoredTransaction>;

    fn into_client_accounts(self) -> Vec<ClientAccount>;

    fn get_transactions_in_dispute(&self) -> Vec<TransactionId>;
}