use std::future::Future;
use tokio::runtime::Runtime;

/// Runs a single future to completion.
///
/// A new instance of tokio Runtime is spun up every time this is called.
/// This is not ideal for production use but a good learning exercise.
fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

fn main() {
    println!("Hello, world!");
}
