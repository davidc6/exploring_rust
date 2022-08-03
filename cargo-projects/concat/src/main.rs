use std::io::{Write, BufReader, Read};
use clap::{arg, Command, ArgAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matching = Command::new("Concat")
        .version("0.1")
        .author("davidc6")
        .arg(arg!([FILE]))
        .arg(arg!(--number <VALUE>).action(ArgAction::Set).required(false))
        .get_matches();

    if matching.get_one::<String>("FILE") != None {
        let filepath_str = matching.get_one::<String>("FILE").unwrap();
        let filepath = std::path::Path::new(filepath_str);

        if filepath.exists() {
            let file = std::fs::File::open(filepath)?;
            let mut buffered = BufReader::new(file);
            let mut buffer = Vec::new();
            buffered.read_to_end(&mut buffer)?;

            std::io::stdout().write_all(&buffer)?;
        }
    }

    if matching.get_one::<String>("number") != None {
        println!("{:?}", matching.get_one::<String>("number"));
    }

    println!("END");
    Ok(())
}
