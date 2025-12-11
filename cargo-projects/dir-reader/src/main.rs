use std::{env, error::Error, fs::{read_dir, ReadDir}, io::Result, iter, path::Path};

// fn traverse_file_tree(space_count: u8, path: &ReadDir) {
//     for (index, entry) in read_dir(path).into_iter().enumerate() {
//         // let dir_entry = entry
//         let path = entry;
// 
//         let full_str = iter::repeat_n(" ", space_count.into());
//         let f: String = full_str.into_iter().collect();
// 
//         if path.read_dir().unwrap().is_dir() {
//             traverse_file_tree(space_count + 2, &path);
//         }
// 
//         let fs = format!("{}-- {:?}", f, entry);
//         println!("{}", fs);
//     }
// }

fn trav(current_directory: Result<ReadDir>, depth: usize) {
    let current_directory = current_directory.unwrap();
    for dir_entry in current_directory {
        let Ok(dir_entry) = dir_entry else {
            panic!("Directory entry is invalid");
        };

        // let path = dir_entry.path();

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

        // println!("{:?}", path);

        if path.is_dir() {
            // let w = format!("{}-- {}", l.join(""), p);
            let w = read_dir(path.as_path());

            // println!("  {}", last_entry_unwrapped);
            trav(w, depth + 2);
        } else {
            // let h = format!("{}-- {}", l.join(""), p);
            // println!("{}", last_entry_unwrapped);
        }
    }
}

fn main() {
    // TODO: Take a directory as an argument to a function as an option.
    // Initially can just work from the directory it runs in.
    let current_directory = Path::new(".").read_dir();

    // traverse_file_tree(0, &a.unwrap());
    trav(current_directory, 2);
}

