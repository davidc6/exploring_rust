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

#[derive(Debug)]
struct Node {
    value: String,
    children: Option<Vec<Box<Node>>>
}

fn traverse_tree(current_directory: Result<ReadDir>, tree: &mut Node) -> &mut Node {
    let Ok(current_directory) = current_directory else {
        panic!("Cannot read current directory");
    };

    // Sort by filename
    let mut dir_entry_collection: Vec<DirEntry> = current_directory.map(|dir_entry| dir_entry.unwrap()).collect();
    dir_entry_collection.sort_by_key(|a| a.file_name());

    for dir_entry in dir_entry_collection {
        let path = dir_entry.path();
        let Some(last_entry) = path.iter().last() else {
            panic!("Cannot get file name");
        };

        let Some(last_entry_unwrapped) = last_entry.to_str() else {
            panic!("Cannot cast entry into a string");
        };

        if path.is_dir() {
            let next_directory = read_dir(path.as_path());

            if let Some(children) = tree.children.as_mut() {
                let new_node = Node {
                    value: last_entry_unwrapped.to_string(), 
                    children: Some(vec![])
                };
                children.push(Box::new(new_node));
            }

            if let Some(children) = tree.children.as_mut() {
                if let Some(last) = children.last_mut() {
                    let n: &mut Node = last;
                    traverse_tree(next_directory, n);
                }
            }
        } else {
            let n = Node { value: last_entry_unwrapped.to_string(), children: None };
            if let Some(children) = tree.children.as_mut() {
                let n = Box::new(n);
                children.push(n);
            }
        }
    }
    
    tree
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
    let tree = traverse_tree(Ok(current_directory), &mut root);
    println!("{:?}", tree);
}

