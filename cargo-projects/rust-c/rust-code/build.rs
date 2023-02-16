extern crate cbindgen;

use std::env;
use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = Config::from_file("cbindgen.toml").unwrap();

    let bindings = cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_config(config)
      .generate();

    bindings
      .expect("Unable to generate bindings")
      .write_to_file("include/bindings.h");
}