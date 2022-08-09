use std::io::{Write, BufReader, Read, stdout, BufRead};
use std::{error::Error, fs::File, path::Path, process};
use clap::{arg, Command, ArgAction, Arg, ArgMatches};

type ReturnType<T> = Result<T, Box<dyn Error>>;

fn init() -> ReturnType<()> {
    let files_arg = Arg::new("verbose").multiple_values(true);
    let matches = Command::new("Concat")
        .version("0.1")
        .author("davidc6")
        .arg(files_arg)
        .arg(arg!(--number).required(false).takes_value(false))
        .get_matches();

    let files = match matches.try_get_many::<String>("verbose") {
        Ok(files) => files.map(|s| s.collect::<Vec<_>>()).unwrap(),
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };

    let should_show_line_numbers = matches.is_present("number");

    let mut count: u8 = 1;
    
    if files.len() > 1 {
        files
            .iter()
            .for_each(|file| {
                let filepath = Path::new(file);

                if filepath.exists() {
                    let file = match File::open(filepath) {
                        Ok(file) => file,
                        Err(err) => {
                            println!("Error: {}", err);
                            process::exit(1);
                        }
                    };

                    let buffered = BufReader::new(file);

                    for line in buffered.lines() {
                        let count_str = count.to_string(); // convert value to String
                        let count_as_bytes = count_str.as_bytes(); // returns byte slice of String contents

                        let b_unwrapped = line.unwrap();
                        let line_as_byte = b_unwrapped.as_bytes();

                        if line_as_byte.len() == 0 && should_show_line_numbers {
                            // println!("empty line");
                            continue;
                        }

                        if should_show_line_numbers {
                            stdout().write(count_as_bytes);
                            stdout().write(b" ");
                        }

                        stdout().write_all(line_as_byte);
                        stdout().write(&"\x0A".as_bytes());

                        count += 1;
                    }
                }
            });
    }
    Ok(())
}

fn main() {
    if let Err(e) = init() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // Ok(())
}
