// use crate::poll::Poll;

mod ffi;
mod poll;

fn main() {
    println!("Hello, world!");

    // TODO
    // let event_queue = Poll::new();
    // let id = 0;
    // event_queue.registy().register(&tcp_stream, id)
    // let mut events = Vec::with_capacity(1);
    // event_queue.poll(&mut events, None);
}
