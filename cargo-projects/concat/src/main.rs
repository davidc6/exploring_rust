use std::{process};
use concat::exec;

fn main() {
    if let Err(e) = exec() {
        println!("{:?}", e);
        process::exit(1);
    }
}
