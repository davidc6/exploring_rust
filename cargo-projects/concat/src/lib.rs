use std::io::{Write, BufReader, stdout, stdin, BufRead};
use std::{error::Error, fs::File, process};
use clap::{Parser};

#[derive(Parser)]
#[clap(name = "Concat", author = "daivdc6", version = "1.0")]
struct Cli {
    /// Path(s) to file(s)
    #[clap(value_parser, required = true)]
    file: Vec<String>,
    /// Optional number lines
    #[clap(short = 'n', long = "number", action)]
    number: bool,
    /// Number nonblank lines
    #[clap(short = 'b', long = "number-nonblank", action)]
    nonblank: bool,
}

type ReturnType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct AppState {
    should_count_empty_lines: bool,
    should_count_non_empty_lines: bool,
    files: Vec<String>,
}

// The compiler does not exactly know the size of BufRead return type
// Hence we allocated memory on the heap by using Box (a pointer with a known size)
fn read_file_and_count_lines(data: Box<dyn BufRead>, should_empty: bool, should_non_empty: bool, counter: &mut u32) -> ReturnType<()> {
    for line in data.lines() {
        let line_raw = line?;
        let line_as_bytes = line_raw.as_bytes();

        let count_str = counter.to_string(); // convert value to String
        let count_as_bytes = count_str.as_bytes(); // returns byte slice of String contents

        if line_as_bytes.is_empty() && should_non_empty {
            continue;
        }

        if should_empty || should_non_empty {
            stdout().write(count_as_bytes);
            stdout().write(b" ");
        }

        stdout().write_all(line_as_bytes);
        stdout().write(&"\x0A".as_bytes());

        // value counter reference points to 
        *counter += 1;
    }

    Ok(())
}

pub fn exec() -> ReturnType<()> {
    let app_state = match retrieve_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut counter: u32 = 1;
    let AppState { should_count_empty_lines, should_count_non_empty_lines, files } = app_state;

    for filename in files {
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}, {}", filename, err),
            Ok(data) => {
                // &mut counter borrows a mutable reference to counter
                match read_file_and_count_lines(
                    data, 
                    should_count_empty_lines, 
                    should_count_non_empty_lines, 
                    &mut counter
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        process::exit(1);
                    }
                }
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
    let cli = Cli::parse();

    Ok(AppState {
        should_count_empty_lines: cli.number,
        should_count_non_empty_lines: cli.nonblank,
        files: cli.file
    })
}
