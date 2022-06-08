use std::io;
use log::SetLoggerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("input csv file must be specified")]
    InputFileRequired,
    #[error("logger setup error: `{0}`")]
    LoggerSetup(#[from] SetLoggerError),
    #[error("input file couldnt be read: `{0}`")]
    InputFile(#[from] io::Error),
    #[error("output couldn't be written to: `{0}`")]
    OutputWriting(#[from] csv_async::Error),
}