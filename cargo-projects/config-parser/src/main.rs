use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[derive(Debug)]
struct Entry<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Debug)]
struct ConfigParser<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> ConfigParser<'a> {
    fn parse(value: &'a str) -> Self {
        let mut entries = Vec::new();

        for line in value.lines() {
            let line = line.trim();

            // Empty lines and comments are ignored
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let line_split = line.split_once(':');

            if let Some((k, v)) = line_split {
                let e = Entry {
                    key: k.trim(),
                    value: v.trim(),
                };

                entries.push(e);
            }
        }

        ConfigParser { entries }
    }

    fn get(&self, key: &'a str) -> Option<&'a str> {
        self.entries
            .iter()
            .find(|entry| entry.key == key)
            .map(|val| val.value)
    }
}

fn main() {
    let string_read = read_file("config.yaml");
    let config_parser = ConfigParser::parse(&string_read);

    println!("{:?}", config_parser.get("port"));
}
