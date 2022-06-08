use crate::models::{ClientAccount, ClientId, DepositData, StoredTransaction, Transaction, WithdrawalData};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use crate::errors::{ProcessorError};
use crate::parser::TransactionSource;
use crate::processor::ProcessingOutput;
use crate::processor::processor_ledger::ProcessorLedger;

pub struct TransactionProcessor<Ledger: ProcessorLedger> {
    ledger: Ledger,
}

impl<Ledger: ProcessorLedger> TransactionProcessor<Ledger> {
    pub fn new(ledger: Ledger) -> Self {
        Self {
            ledger,
        }
    }

    fn get_or_create_unlocked_client(&mut self, client_id: ClientId) -> Result<&mut ClientAccount, ProcessorError> {
        let client = self.ledger.get_or_create_client(client_id);

        if client.is_locked() {
            Err(ProcessorError::ClientLocked(client_id))
        } else {
            Ok(client)
        }
    }

    fn process_deposit(&mut self, deposit: DepositData) -> Result<(), ProcessorError> {
        let client_account = self.get_or_create_unlocked_client(deposit.client_id)?;

        client_account.add_available(deposit.amount);
        self.ledger.store_transaction(deposit.transaction_id, StoredTransaction::new(deposit));

        Ok(())
    }

    fn process_withdrawal(&mut self, withdrawal: WithdrawalData) -> Result<(), ProcessorError> {
        let client_account = self.get_or_create_unlocked_client(withdrawal.client_id)?;

        if client_account.get_available() < withdrawal.amount {
            return Err(ProcessorError::InsufficientFunds(
                withdrawal.client_id,
                withdrawal.transaction_id,
                withdrawal.amount
            ));
        }

        client_account.remove_available(withdrawal.amount);
        Ok(())
    }

    fn process_transaction(&mut self, transaction: Transaction) -> Result<(), ProcessorError> {
        match transaction {
            Transaction::Deposit(deposit) => self.process_deposit(deposit),
            Transaction::Withdrawal(withdrawal) => self.process_withdrawal(withdrawal),
            _ => todo!(),
        }
    }

    pub async fn consume_source(&mut self, source: impl TransactionSource) {
        let txs = source
            .stream_transactions();
        pin_mut!(txs);

        while let Some(transaction_res) = txs.next().await {
            match transaction_res {
                Ok(transaction) => {
                    self.process_transaction(transaction)
                        .err()
                        .map(|error| log::warn!("error while processing transaction: {}", error));
                },
                Err(error) => log::warn!("found unprocessable transaction: {:?}", error),
            }
        }
    }

    pub fn into_output(self) -> ProcessingOutput {
        ProcessingOutput {
            clients: self.ledger.into_client_accounts(),
        }
    }
}
