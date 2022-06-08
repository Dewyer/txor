mod setup_logs;
mod take_result_and_exit;

use std::env;
use log::info;
use crate::errors::{CliError, TxorError};
use crate::parser::CsvTransactionSource;
use crate::processor::{InMemoryProcessorLedger, TransactionProcessor};

async fn run_cli_main() -> Result<(), TxorError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_verbose = args.iter().any(|el| el == "--verbose");
    setup_logs::setup_logs(is_verbose)?;

    let input_file_path = args.get(0)
        .ok_or::<TxorError>(CliError::InputFileRequired.into())?;

    let input_file = tokio::fs::File::open(input_file_path)
        .await
        .map_err(|err| TxorError::Cli(CliError::InputFile(err)))?;

    let txs = CsvTransactionSource::from_reader(input_file);
    let mut processor = TransactionProcessor::new(
        InMemoryProcessorLedger::new(),
    );
    processor.consume_source(txs).await;

    Ok(())
}

pub async fn cli_main() -> ! {
    take_result_and_exit::take_result_and_exit(
        run_cli_main().await
    );
}