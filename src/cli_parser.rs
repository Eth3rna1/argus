/*
    Setting up the use of environment variables and command line arguments
*/
use crate::method::Method;

use std::env;
use std::process::exit;

pub fn get_env_var(var: &str) -> Result<String, String> {
    match env::var(var) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Environment variable \"{}\" was not set.", var)),
    }
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_method() -> Result<Method, String> {
    let args = get_args();
    if args.len() <= 1 {
        eprintln!("Please provide a method. [file | email]");
        exit(1);
    }
    match args[1].as_str() {
        "email" => {
            let email = get_env_var("EMAIL")?;
            let password = get_env_var("PASSWORD")?;
            let recipient = get_env_var("RECIPIENT")?;
            Ok(Method::Email {
                email,
                password,
                recipient,
            })
        }
        "file" => {
            let path = get_env_var("PATH")?;
            Ok(Method::File { path })
        }
        _ => return Err("Not a supported method".to_string()),
    }
}
