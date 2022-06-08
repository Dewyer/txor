use crate::errors::TxorError;

pub fn setup_logs(is_verbose: bool) -> Result<(), TxorError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(if is_verbose {
            log::LevelFilter::Info
        } else {
            log::LevelFilter::Error
        })
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .map_err(|err| TxorError::Cli(err.into()))
}
