mod cli;
mod errors;
mod models;
mod parser;
mod processor;
mod utils;

#[tokio::main]
async fn main() {
    cli::cli_main().await
}
