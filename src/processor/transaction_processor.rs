use std::collections::HashMap;
use crate::models::{ClientAccount, ClientId, Transaction, TransactionId};
use crate::TransactionSource;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub struct TransactionProcessor {
    ledger: HashMap<ClientId, ClientAccount>,
    processed_transaction: HashMap<TransactionId, Transaction>,
}

impl TransactionProcessor {
    pub fn new() -> Self {
        Self {
            ledger: HashMap::new(),
            processed_transaction: HashMap::new(),
        }
    }

    pub async fn consume_source(&mut self, source: TransactionSource) {
        let txs = source.stream_transactions();
        pin_mut!(txs);

        while let Some(value) = txs.next().await {
            log::info!("got {:?}", value);
        }
    }
}
