//! main.rs

use bookreview::run;

// async runtime is loaded on top of the main fn
// and used to drive futures (async computations) to completion
// tokio runtime takes async code in the main fn and runs it
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // if run errors, the error will bubble up
    run()?.await
}
