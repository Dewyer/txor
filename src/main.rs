use std::env;

mod processor;
mod models;
mod parser;
mod errors;
mod cli;
mod utils;

#[tokio::main]
async fn main() {
    cli::cli_main().await
}