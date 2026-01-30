use std::{io::Result, net::TcpStream};
use crate::ffi;

/// This is layer over epoll.
///
/// epoll (event poll) is referred to as an I/O event notification facility.
/// It is Linux-specific but it is not POSIX.
///
/// It is used to monitor multiple file descriptors to check if they are ready for I/O.

type Events = Vec<ffi::Event>;

/// Poll represents the Event Queue.
pub struct Poll {
    registry: Registry
}

impl Poll {
    /// Creates an event queue.
    pub fn new() -> Self {
        todo!()
    }

    /// Returns a reference to a registry . It is used as a handle to register interest about new 
    /// events.
    fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thread while polling until the event is ready or times out.
    fn poll(&mut self, events: &mut Events, timeout: Option<i32>) {
        todo!()
    }
}

/// Registry allows us to register interest in new events.
/// This will indicate what kind of events we want Poll (our event queue) to keep track of.
pub struct Registry {
    raw_file_descriptor: i32
}

impl Registry {
    pub fn register(&self, src: &TcpStream, token: usize, interests: i32) -> Result<()> {
        todo!()
    }

    /// Creates an independenly owned registry
    ///
    /// This will give users an option to get an owned version of Registry.
    /// The instance can be shared as well as passed to other threads. This 
    /// will allow other threads to register interest to the same Poll instance.
    ///
    /// This is not implemented for now.
    pub fn try_clone(&self) {
        unimplemented!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}

