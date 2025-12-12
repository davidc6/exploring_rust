use std::{env, fs::{read_dir, ReadDir}, io::Result, iter, path::Path};

fn traverse_file_tree(current_directory: Result<ReadDir>, depth: usize) {
    let current_directory = current_directory.unwrap();
    for dir_entry in current_directory {
        let Ok(dir_entry) = dir_entry else {
            panic!("Directory entry is invalid");
        };

        let path = dir_entry.path();
        let last_entry = path.iter().last().unwrap();
        let last_entry_unwrapped = last_entry.to_str().unwrap();

        let l = iter::repeat_n(" ", depth);
        let l: Vec<_> = l.collect();
        let w = format!("{}{}", l.join(""), last_entry_unwrapped);

        println!("{}", w);

        let Ok(current_dir) = env::current_dir() else {
            panic!("Current path incorrect");
        };

        if path.is_dir() {
            let w = read_dir(path.as_path());
            traverse_file_tree(w, depth + 4);
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

