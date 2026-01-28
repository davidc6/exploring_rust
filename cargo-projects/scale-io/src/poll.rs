use std::net::TcpStream;
use crate::ffi;

type Events = Vec<ffi::Event>;

/// Poll represents the event queue.
pub struct Poll {
    registry: Registry
}

impl Poll {
    /// Creates an event queue.
    fn new() -> Self {
        todo!()
    }

    /// Returns a registry to register interest about events.
    fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thread while polling 
    /// until the event is ready or times out.
    fn poll(&mut self, events: &mut Events, timeout: Option<i32>) {
        todo!()
    }
}

pub struct Registry {
    raw_file_descriptor: i32
}

impl Registry {
    pub fn register(&self, src: &TcpStream, token: usize, interests: i32) -> Result<()> {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}

