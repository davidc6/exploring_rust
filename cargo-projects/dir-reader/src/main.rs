use std::{error::Error, fs::{read_dir, DirEntry, ReadDir}, io::Result, iter, path::Path};

fn traverse_file_tree(current_directory: Result<ReadDir>, line_to_print: String) {
    let Ok(current_directory) = current_directory else {
        panic!("Cannot read current directory");
    };

    // Sort by filename
    let mut dir_entry_collection: Vec<DirEntry> = current_directory.map(|dir_entry| dir_entry.unwrap()).collect();
    dir_entry_collection.sort_by_key(|a| a.file_name());
    let len = dir_entry_collection.len();
    let mut cur_count = 1;

    for dir_entry in dir_entry_collection {

        let path = dir_entry.path();
        let Some(last_entry) = path.iter().last() else {
            panic!("Cannot get file name");
        };

        let Some(last_entry_unwrapped) = last_entry.to_str() else {
            panic!("Cannot cast entry into a string");
        };

        // let spacing = iter::repeat_n(" ", line_to_print).collect::<Vec<_>>().join("");
        // let value = format!("{}{}", spacing, last_entry_unwrapped);
        let value = format!("{}{}", line_to_print, last_entry_unwrapped);

        if path.is_dir() {
            let next_directory = read_dir(path.as_path());

            let future_print = if cur_count + 1 == len {
                format!("{}{}", line_to_print, "   ")
            } else {
                format!("{}{}", line_to_print, "|  ")
            };

            println!("{}", value);
            traverse_file_tree(next_directory, future_print);
        } else {
            // let future_print = format!("{}{}", "-", line_to_print);
            println!("{}- {}", line_to_print, last_entry_unwrapped);
        }

        cur_count += 1;
    }
}

fn main() {
    // TODO: Take a directory as an argument to a function as an option.
    // Initially can just work from the directory it runs in.
    let current_directory = Path::new(".").read_dir();

    // traverse_file_tree(0, &a.unwrap());
    traverse_file_tree(current_directory, String::new());
}

