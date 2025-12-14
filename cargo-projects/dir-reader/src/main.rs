use std::{fs::{read_dir, ReadDir}, io::Result, iter, path::Path};

fn traverse_file_tree(current_directory: Result<ReadDir>, depth: usize) {
    let Ok(current_directory) = current_directory else {
        panic!("Cannot read current directory");
    };

    for dir_entry in current_directory {
        let Ok(dir_entry) = dir_entry else {
            panic!("Directory entry is invalid");
        };

        let path = dir_entry.path();
        let Some(last_entry) = path.iter().last() else {
            panic!("Cannot get file name");
        };

        let Some(last_entry_unwrapped) = last_entry.to_str() else {
            panic!("Cannot cast entry into a string");
        };

        let spacing = iter::repeat_n(" ", depth).collect::<Vec<_>>();
        let value = format!("{}{}", spacing.join(""), last_entry_unwrapped);

        println!("{}", value);

        if path.is_dir() {
            let next_directory = read_dir(path.as_path());
            traverse_file_tree(next_directory, depth + 4);
        }
    }
}

fn main() {
    // TODO: Take a directory as an argument to a function as an option.
    // Initially can just work from the directory it runs in.
    let current_directory = Path::new(".").read_dir();

    // traverse_file_tree(0, &a.unwrap());
    traverse_file_tree(current_directory, 0);
}

