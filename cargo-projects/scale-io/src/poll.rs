use std::net::TcpStream;

/// This is the event queue.
pub struct Poll {
    registry: Registry
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

