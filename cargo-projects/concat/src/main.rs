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
        .arg(arg!(--nonblank).required(false).takes_value(false).long("number-nonblank"))
        .get_matches();

    let files = match matches.try_get_many::<String>("verbose") {
        Ok(files) => files.map(|s| s.collect::<Vec<_>>()).unwrap(),
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };

    let should_show_blank = matches.is_present("number");
    let should_show_non_blank = matches.is_present("nonblank");

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

                        if line_as_byte.is_empty() && should_show_non_blank {
                            continue;
                        }

                        if should_show_blank || should_show_non_blank {
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

#[derive(Debug)]
struct AppState {
    should_count_empty_lines: bool,
    should_count_non_empty_lines: bool,
    files: Vec<String>,
}

fn retrieve_args() -> ReturnType<AppState> {
    let files_arg = Arg::new("verbose").multiple_values(true);
    let matches = Command::new("Concat")
        .version("0.1")
        .author("davidc6")
        .arg(files_arg)
        .arg(arg!(--number).required(false).takes_value(false))
        .arg(arg!(--nonblank).required(false).takes_value(false).long("number-nonblank"))
        .get_matches();

    let files = match matches.try_get_many::<String>("verbose") {
        Ok(files) => files.unwrap(),
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };

    Ok(AppState {
        should_count_empty_lines: matches.is_present("number"),
        should_count_non_empty_lines: matches.is_present("nonblank"),
        files: files.map(String::from).collect::<Vec<String>>()
    })
}

fn main() {
    let app_state = match retrieve_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("{:?}", app_state);

    if let Err(e) = init() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
