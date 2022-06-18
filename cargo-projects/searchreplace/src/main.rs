use std::{env, fs};

#[derive(Debug)]
struct Args {
    search: String,
    replace: String,
    input: String,
    output: String
}

fn print_err() {
    eprintln!("{} Usage: sr <target> <replacement> <input> <output>\n", "Error:");
}

fn parse() -> Args {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_err();
        std::process::exit(1);
    }

    // we use clone() to copy the values and leave the original values in place
    Args { search: args[0].clone(), replace: args[1].clone(), input: args[2].clone(), output: args[3].clone() }
}

fn main() {
    let args = parse();

    // read data from file
    let data = match fs::read_to_string(&args.input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} could not read from file '{}': {:?}", "Error:", args.input, e);
            std::process::exit(1);
        }
    };

    // write data to file
    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} could not write to file '{}': {:?}", "Error:", args.output, e);
            std::process::exit(1);
        }
    }

    println!("{:?}", args);
}