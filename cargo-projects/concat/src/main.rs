use std::io::{Write, BufReader, Read, stdout, stdin, BufRead};
use std::{error::Error, fs::File, path::Path, process};
use clap::{arg, Command, ArgAction, Arg, ArgMatches, App};

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

// The compiler does not exactly know the size of BufRead return type
// Hence we allocated memory on the heap by using Box (a pointer with a known size)
fn read_file_and_count_lines(data: Box<dyn BufRead>, should_empty: bool, should_non_empty: bool, counter: &mut u32) {
    for line in data.lines() {
        let line_raw = line.unwrap();
        let line_as_bytes = line_raw.as_bytes();

        // no flags
        if should_empty == false && should_non_empty == false {
            stdout().write(line_as_bytes);
            stdout().write(&"\x0A".as_bytes());
            continue;
        }

        // count empty line
        if line_as_bytes.is_empty() && should_empty {
            *counter += 1;
            let str = counter.to_string();
            stdout().write(str.as_bytes());
            stdout().write(&"\x0A".as_bytes());
            continue;
        }

        if line_as_bytes.is_empty() && should_non_empty {
            continue;
        }

        // count non-empty lines
        *counter += 1;
        let str = counter.to_string();
        stdout().write(str.as_bytes());
        stdout().write(b" ");
        stdout().write(line_as_bytes);
        stdout().write(&"\x0A".as_bytes());
    }
}

fn exec(args: AppState) -> ReturnType<()> {
    let mut counter: u32 = 0;
    let AppState { should_count_empty_lines, should_count_non_empty_lines, files } = args;

    for filename in files {
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}, {}", filename, err),
            Ok(data) => {
                read_file_and_count_lines(data, should_count_empty_lines, should_count_non_empty_lines, &mut counter);
            }
        }
    }

    Ok(())
}

// Box is necessary to hold the filehandle on the heap
fn open_file(filename: &str) -> ReturnType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
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
    let mut app_state = match retrieve_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    exec(app_state);

    // if let Err(e) = init() {
    //     eprintln!("{}", e);
    //     std::process::exit(1);
    // }
}
