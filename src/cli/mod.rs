#[cfg(test)]
pub mod fixture_test;
mod setup_logs;
mod take_result_and_exit;
mod write_processing_output;

use crate::errors::{CliError, TxorError};
use crate::parser::CsvTransactionSource;
use crate::processor::{InMemoryProcessorLedger, ProcessingOutput, TransactionProcessor};
use std::env;
use tokio::fs::File;
use tokio::io;

pub async fn process_reader(
    input_reader: impl io::AsyncRead + 'static + Send + Unpin,
) -> ProcessingOutput {
    let txs = CsvTransactionSource::from_reader(input_reader);
    let mut processor = TransactionProcessor::new(InMemoryProcessorLedger::new());
    processor.consume_source(txs).await;

    processor.into_output()
}

async fn run_cli_main() -> Result<(), TxorError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let is_verbose = args.iter().any(|el| el == "--verbose");
    setup_logs::setup_logs(is_verbose)?;

    let input_file_path = args
        .get(0)
        .ok_or::<TxorError>(CliError::InputFileRequired.into())?;

    let input_file = File::open(input_file_path)
        .await
        .map_err(|err| TxorError::Cli(CliError::InputFile(err)))?;

    let output = process_reader(input_file).await;

    write_processing_output::write_processing_output(output, tokio::io::stdout()).await?;
    Ok(())
}

pub async fn cli_main() -> ! {
    take_result_and_exit::take_result_and_exit(run_cli_main().await);
}
