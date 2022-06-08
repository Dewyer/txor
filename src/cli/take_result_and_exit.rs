use log::error;
use std::fmt::Display;
use std::process;

pub fn take_result_and_exit<T, E: Display>(result: Result<T, E>) -> ! {
    match result {
        Ok(_) => process::exit(0),
        Err(err) => {
            error!("wtf {}", err);
            process::exit(1)
        }
    }
}
