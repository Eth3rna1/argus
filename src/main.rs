mod cli_parser;
mod argus;
mod method;
use argus::Argus;

use std::process::exit;

const BUFFER_CAPACITY: usize = 100;

fn wrapper() -> Result<(), String> {
    let method = cli_parser::get_method()?;
    let mut argus = Argus::new(method)
        .set_buffer_capacity(BUFFER_CAPACITY);
    argus.start();
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
