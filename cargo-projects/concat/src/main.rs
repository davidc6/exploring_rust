use std::io::{Write, BufReader, Read, stdout, BufRead};
use std::{error::Error, fs::File, path::Path, process};
use clap::{arg, Command, ArgAction, Arg, ArgMatches};

// fn files(matches: &ArgMatches) ->  {
//     return match matches.try_get_many::<String>("verbose") {
//         Ok(files) => files.map(|s| s.collect::<Vec<_>>()).unwrap(),
//         Err(err) => {
//             println!("{:?}", err);
//             process::exit(1);
//         }
//     };
// }

fn main() -> Result<(), Box<dyn Error>> {
    let files_arg = Arg::new("verbose").multiple_values(true);
    let matches = Command::new("Concat")
        .version("0.1")
        .author("davidc6")
        .arg(files_arg)
        .arg(arg!(--number).required(false).takes_value(false))
        .get_matches();

    // let files = matching.get_many::<Vec<_>>("FILE").unwrap();
    // println!("{:?}", files);
    // let files: Vec<_> = matching.values_of("verbose1").unwrap().collect();

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
        let vec: Vec<u8> = files
            .iter()
            .map(|file| {
                let filepath = Path::new(file);
                // let mut buffer = Vec::new();

                // if filepath.exists() {
                    let file = match File::open(filepath) {
                        Ok(file) => file,
                        Err(err) => {
                            println!("Error: {}", err);
                            process::exit(1);
                        }
                    };

                    let mut buffered = BufReader::new(file);

                    // match buffered.read_to_end(&mut buffer) {
                    //     Ok(_) => (),
                    //     Err(err) => {
                    //         println!("Error: {}", err);
                    //         process::exit(1);
                    //     }
                    // };

                    for line in buffered.lines() {
                        let count_str = count.to_string(); // convert value to String
                        let count_as_bytes = count_str.as_bytes(); // returns byte slice of String contents

                        let b_unwrapped = line.unwrap();
                        let line_as_byte = b_unwrapped.as_bytes();

                        println!("{:?}", should_show_line_numbers);

                        if line_as_byte.len() == 0 && should_show_line_numbers {
                            println!("empty line");
                            continue;
                        }


                        if should_show_line_numbers {
                            stdout().write(count_as_bytes);
                            stdout().write(b" ");
                        }

                        stdout().write_all(line_as_byte);
                        stdout().write(&"\x0A".as_bytes());

                        count += 1;

                        // match c {
                        //     Ok(_) => (),
                        //     Err(err) => {
                        //         println!("Error: {}", err);
                        //         process::exit(1);
                        //     }
                        // }
                    }

                    // match stdout().write_all(&buffer) {
                    //     Ok(_) => (),
                    //     Err(err) => {
                    //         println!("Error: {}", err);
                    //         process::exit(1);
                    //     }
                    // };

                    return 1;
                // }

                // return buffer;
            })
            .collect();

    }


    // let files = matching
    //     .try_get_many::<String>("verbose")
    //     .map(|s| s.unwrap().collect::<Vec<_>>())
    //     .unwrap();
    // println!("{:?}", files);

    // if matching.get_many("FILE") != None {
    // }

    // need to get all files
    // if matching.get_one::<String>("FILE") != None {
    //     let filepath_str = matching.get_one::<String>("FILE").unwrap();
    //     let filepath = Path::new(filepath_str);

    //     if filepath.exists() {
    //         let file = match File::open(filepath) {
    //             Ok(file) => file,
    //             Err(err) => {
    //                 println!("Error: {}", err);
    //                 process::exit(1);
    //             }
    //         };

    //         let mut buffered = BufReader::new(file);
    //         let mut buffer = Vec::new();

    //         match buffered.read_to_end(&mut buffer) {
    //             Ok(_) => (),
    //             Err(err) => {
    //                 println!("Error: {}", err);
    //                 process::exit(1);
    //             }
    //         };

    //         match stdout().write_all(&buffer) {
    //             Ok(_) => (),
    //             Err(err) => {
    //                 println!("Error: {}", err);
    //                 process::exit(1);
    //             }
    //         };
    //     }
    // }

    // if matching.get_one::<String>("number") != None {
    //     println!("{:?}", matching.get_one::<String>("number"));
    // }

    Ok(())
}
