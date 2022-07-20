use clap::{App, Arg};

fn main() {
    let matches = App::new("cliapp")
        .version("0.1.0")
        .author("David C")
        .about("Just a test CLI app")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let t = "text";
    let text = matches.get_many::<String>(t)
        .unwrap()
        .cloned()
        .collect::<Vec<_>>();

    let omit_newline = matches.contains_id("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
