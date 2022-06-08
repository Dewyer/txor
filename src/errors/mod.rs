mod cli_error;
mod parser_error;
mod processor_error;

pub use cli_error::*;
pub use parser_error::*;
pub use processor_error::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TxorError {
    #[error("cli error: `{0}`")]
    Cli(#[from] CliError),
    #[error("parser error: `{0}`")]
    Parser(#[from] ParserError),
    #[error("processor error: `{0}`")]
    Processor(#[from] ProcessorError),
}
