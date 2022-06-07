use tokio::fs::File;
use crate::parser::TransactionSource;
use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

mod models;
mod parser;
mod errors;

#[tokio::main]
async fn main() {
    let ff = File::open("test_data/test.csv").await.unwrap();
    let txs = TransactionSource::from_reader(ff)
        .stream_transactions();
    pin_mut!(txs);

    while let Some(value) = txs.next().await {
        println!("got {:?}", value);
    }
}