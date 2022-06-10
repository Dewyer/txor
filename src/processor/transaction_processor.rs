use crate::errors::ProcessorError;
use crate::models::{
    ChargebackData, ClientAccount, ClientId, DepositData, DisputeData, DisputeState,
    ResolutionData, StoredTransaction, Transaction, TransactionId, WithdrawalData,
};
use crate::parser::TransactionSource;
use crate::processor::processor_ledger::ProcessorLedger;
use crate::processor::ProcessingOutput;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub struct TransactionProcessor<Ledger: ProcessorLedger> {
    ledger: Ledger,
}

impl<Ledger: ProcessorLedger> TransactionProcessor<Ledger> {
    pub fn new(ledger: Ledger) -> Self {
        Self { ledger }
    }

    fn get_or_create_unlocked_client(
        &mut self,
        client_id: ClientId,
    ) -> Result<&mut ClientAccount, ProcessorError> {
        let client = self.ledger.get_or_create_client(client_id);

        if client.is_locked() {
            Err(ProcessorError::ClientLocked(client_id))
        } else {
            Ok(client)
        }
    }

    fn assert_transaction_unique(
        &self,
        transaction_id: TransactionId,
    ) -> Result<(), ProcessorError> {
        if self.ledger.has_stored_transaction(transaction_id) {
            Err(ProcessorError::TransactionAlreadyExists(transaction_id))
        } else {
            Ok(())
        }
    }

    fn assert_client_has_access_to_transaction(
        client_account: &ClientAccount,
        stored_tx: &StoredTransaction,
    ) -> Result<(), ProcessorError> {
        if client_account.get_id() != stored_tx.get_data().client_id {
            Err(ProcessorError::ClientInsufficientAccess(
                client_account.get_id(),
                stored_tx.get_data().transaction_id,
            ))
        } else {
            Ok(())
        }
    }

    fn get_client_and_referenced_transaction(
        &mut self,
        client_id: ClientId,
        transaction_id: TransactionId,
        enforce_dispute_state: DisputeState,
    ) -> Result<(&mut ClientAccount, StoredTransaction), ProcessorError> {
        let stored_tx = self
            .ledger
            .get_stored_transaction(transaction_id)
            .ok_or(ProcessorError::TransactionDoesntExists(transaction_id))?
            .clone();
        let client_account = self.get_or_create_unlocked_client(client_id)?;
        Self::assert_client_has_access_to_transaction(&client_account, &stored_tx)?;

        if stored_tx.get_dispute_state() != &enforce_dispute_state {
            if enforce_dispute_state == DisputeState::Undisputed {
                return Err(ProcessorError::TransactionAlreadyDisputed(transaction_id));
            }

            if enforce_dispute_state == DisputeState::Disputed {
                return Err(ProcessorError::TransactionNotDisputed(transaction_id));
            }
        }

        Ok((client_account, stored_tx))
    }

    fn process_deposit(&mut self, deposit: DepositData) -> Result<(), ProcessorError> {
        self.assert_transaction_unique(deposit.transaction_id)?;
        let client_account = self.get_or_create_unlocked_client(deposit.client_id)?;

        client_account.add_available(deposit.amount)?;
        self.ledger
            .store_transaction(deposit.transaction_id, StoredTransaction::new(deposit));

        Ok(())
    }

    fn process_withdrawal(&mut self, withdrawal: WithdrawalData) -> Result<(), ProcessorError> {
        self.assert_transaction_unique(withdrawal.transaction_id)?;
        let client_account = self.get_or_create_unlocked_client(withdrawal.client_id)?;

        if client_account.get_available() < withdrawal.amount {
            return Err(ProcessorError::InsufficientFunds(
                withdrawal.client_id,
                withdrawal.transaction_id,
                withdrawal.amount,
            ));
        }

        client_account.remove_available(withdrawal.amount)?;
        Ok(())
    }

    fn process_dispute(&mut self, dispute: DisputeData) -> Result<(), ProcessorError> {
        let (client_account, stored_tx) = self.get_client_and_referenced_transaction(
            dispute.client_id,
            dispute.referenced_transaction_id,
            DisputeState::Undisputed,
        )?;

        client_account.hold(stored_tx.get_data().amount)?;

        let stored_tx = self
            .ledger
            .get_stored_transaction_mut(dispute.referenced_transaction_id)
            .ok_or(ProcessorError::TransactionDoesntExists(
                dispute.referenced_transaction_id,
            ))?;
        stored_tx.dispute();

        Ok(())
    }

    fn process_resolution(&mut self, resolution: ResolutionData) -> Result<(), ProcessorError> {
        let (client_account, stored_tx) = self.get_client_and_referenced_transaction(
            resolution.client_id,
            resolution.referenced_transaction_id,
            DisputeState::Disputed,
        )?;

        client_account.un_hold(stored_tx.get_data().amount)?;

        let stored_tx = self
            .ledger
            .get_stored_transaction_mut(resolution.referenced_transaction_id)
            .ok_or(ProcessorError::TransactionDoesntExists(
                resolution.referenced_transaction_id,
            ))?;
        stored_tx.remove_dispute();

        Ok(())
    }

    fn process_chargeback(&mut self, chargeback: ChargebackData) -> Result<(), ProcessorError> {
        let (client_account, stored_tx) = self.get_client_and_referenced_transaction(
            chargeback.client_id,
            chargeback.referenced_transaction_id,
            DisputeState::Disputed,
        )?;

        let chargeback_am = stored_tx.get_data().amount;
        client_account.un_hold(chargeback_am)?;
        client_account.remove_available(chargeback_am)?;
        client_account.lock();

        let stored_tx = self
            .ledger
            .get_stored_transaction_mut(chargeback.referenced_transaction_id)
            .ok_or(ProcessorError::TransactionDoesntExists(
                chargeback.referenced_transaction_id,
            ))?;
        stored_tx.remove_dispute();

        Ok(())
    }

    fn process_transaction(&mut self, transaction: Transaction) -> Result<(), ProcessorError> {
        match transaction {
            Transaction::Deposit(deposit) => self.process_deposit(deposit),
            Transaction::Withdrawal(withdrawal) => self.process_withdrawal(withdrawal),
            Transaction::Dispute(dispute) => self.process_dispute(dispute),
            Transaction::Resolution(resolution) => self.process_resolution(resolution),
            Transaction::Chargeback(chargeback) => self.process_chargeback(chargeback),
        }
    }

    pub async fn consume_source(&mut self, source: impl TransactionSource) {
        let txs = source.stream_transactions();
        pin_mut!(txs);

        while let Some(transaction_res) = txs.next().await {
            match transaction_res {
                Ok(transaction) => {
                    self.process_transaction(transaction)
                        .err()
                        .map(|error| log::warn!("error while processing transaction: {}", error));
                }
                Err(error) => log::warn!("found unprocessable transaction: {:?}", error),
            }
        }
    }

    pub fn into_output(self) -> ProcessingOutput {
        let tx_disputed = self.ledger.get_transactions_in_dispute();
        ProcessingOutput {
            clients: self.ledger.into_client_accounts(),
            transactions_in_dispute: tx_disputed,
        }
    }
}
