use std::io::{Write, BufReader, Read, stdout};
use std::{error::Error, fs::File, path::Path, process};
use clap::{arg, Command, ArgAction};



fn main() -> Result<(), Box<dyn Error>> {
    let matching = Command::new("Concat")
        .version("0.1")
        .author("davidc6")
        .arg(arg!([FILE]))
        .arg(arg!(--number <VALUE>).action(ArgAction::Set).required(false))
        .get_matches();

    if matching.get_one::<String>("FILE") != None {
        let filepath_str = matching.get_one::<String>("FILE").unwrap();
        let filepath = Path::new(filepath_str);

        if filepath.exists() {
            let file = match File::open(filepath) {
                Ok(file) => file,
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            };

            let mut buffered = BufReader::new(file);
            let mut buffer = Vec::new();

            match buffered.read_to_end(&mut buffer) {
                Ok(_) => (),
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            };

            match stdout().write_all(&buffer) {
                Ok(_) => (),
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            };
        }
    }

    if matching.get_one::<String>("number") != None {
        println!("{:?}", matching.get_one::<String>("number"));
    }

    println!("END");
    Ok(())
}
