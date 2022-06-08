use std::collections::HashMap;
use crate::models::{ClientAccount, ClientId, Transaction, TransactionId};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use crate::parser::TransactionSource;
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

    pub async fn consume_source(&mut self, source: impl TransactionSource) {
        let txs = source
            .stream_transactions();
        pin_mut!(txs);

        while let Some(value) = txs.next().await {
            log::info!("got {:?}", value);
        }
    }
}
