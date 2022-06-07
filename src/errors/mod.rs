mod parser_error;

pub use parser_error::*;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum TxorError {
    #[error("parser error: `{0}`")]
    Parser(#[from] ParserError),
}