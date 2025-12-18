use std::{error::Error, fs::{read_dir, DirEntry, ReadDir}, io::Result, iter, path::Path};

struct File {
    name: String
}

struct Dir {
    name: String,
    children: Vec<Entities>
}

#[derive(Debug)]
enum Entities {
    Dir {
        name: String,
        children: Vec<Entities>
    },
    File {
        name: String
    }
}

fn traverse_tree(current_directory: Result<ReadDir>, node: &mut Node) -> &mut Node {
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
        // let value = format!("{}{}", line_to_print, last_entry_unwrapped);

        if path.is_dir() {
            let next_directory = read_dir(path.as_path());

            let n = Node { value: last_entry_unwrapped.to_string(), children: Some(vec![]) };

            if let Some(children) = node.children.as_mut() {
                children.push(Box::new(n));
            }

            if let Some(children) = node.children.as_mut() {
                if let Some(last) = children.last_mut() {
                    let n: &mut Node = last;
                    traverse_tree(next_directory, n);
                }

               //  if let Some(last) = children.last().as_mut() {
               //      let mut n = last;
               //      traverse_tree(next_directory, &mut n);
               //  }
            }

        } else {
            let n = Node { value: last_entry_unwrapped.to_string(), children: None };
            if let Some(children) = node.children.as_mut() {
                let n = Box::new(n);
                children.push(n);
            }

        }

        cur_count += 1;
    }
    
    node
}

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

            let future_print = if cur_count == len {
                format!("{}{}", line_to_print, "   ")
            } else {
                format!("{}{}", line_to_print, " ")
            };

            // println!("{}", format!("{}{}{}", "\x1b[34m", value, "\x1b[0m"));
            println!("{}", last_entry_unwrapped);
            traverse_file_tree(next_directory, future_print);
        } else {
            // let future_print = format!("{}{}", "-", line_to_print);
            // println!("\u{2500} \u{2502}\u{2502} {} {}", line_to_print, last_entry_unwrapped);
            println!("{}", last_entry_unwrapped);
        }

        cur_count += 1;
    }

}

#[derive(Debug)]
struct Node {
    value: String,
    children: Option<Vec<Box<Node>>>
}

fn main() {
    // TODO: Take a directory as an argument to a function as an option.
    // Initially can just work from the directory it runs in.
    let Ok(current_directory) = Path::new(".").read_dir() else {
        panic!("Cannot get root");
    };

    let mut root = Node {
        value: ".".to_owned(),
        children: Some(vec![])
    };

    let node = traverse_tree(Ok(current_directory), &mut root);
    println!("{:?}", node);

    // traverse_file_tree(current_directory, String::new());
}

