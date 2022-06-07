use tokio::fs::File;
use crate::parser::TransactionSource;

mod processor;
mod models;
mod parser;
mod errors;
mod cli;

#[tokio::main]
async fn main() {
    cli::cli_main().await
}