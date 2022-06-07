mod parser_error;
mod cli_error;

pub use cli_error::*;
pub use parser_error::*;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum TxorError {
    #[error("cli error: `{0}`")]
    Cli(#[from] CliError),
    #[error("parser error: `{0}`")]
    Parser(#[from] ParserError),
}