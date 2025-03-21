/*
    CLI parser and makes sure the environment variables are set up correctly
*/
use crate::method::Method;

use std::env;
use std::process::exit;

/// Obtains the environment variable, but when an error variant
/// is returned, a custom error message will be constructed and return
pub fn get_env_var(var: &str) -> Result<String, String> {
    match env::var(var) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Environment variable \"{}\" was not set.", var)),
    }
}

/// Obtains the command line variables
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

/// Indexes into the 1th index from the command line
/// variables and upon evaluation, a method will be
/// constructed and returned, otherwise, a custom error message
pub fn get_method() -> Result<Method, String> {
    let args = get_args();
    if args.len() <= 1 {
        return Err("Please provide a method. [file | email]".to_string());
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
            let path = get_env_var("FILE")?;
            Ok(Method::File { path })
        }
        _ => return Err("Not a supported method".to_string()),
    }
}
