use crate::build::method::Method;


pub struct Keylogger {
    method : Method,
    buffer_capacity : usize,
    length : usize
}

impl Keylogger {
    pub fn new(method : Method) -> Self {
        Self {
            method,
            buffer_capacity : 0,
            length : 0
        }
    }

    pub fn start(&mut self) {
        todo!()
    }
}
