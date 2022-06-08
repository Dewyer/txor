use crate::models::{ClientAccount, ClientId, StoredTransaction, TransactionId};

pub trait ProcessorLedger {
    fn get_or_create_client(&mut self, client_id: ClientId) -> &mut ClientAccount;

    fn store_transaction(&mut self, transaction_id: TransactionId, transaction: StoredTransaction);

    fn into_client_accounts(self) -> Vec<ClientAccount>;
}