use std::{env, fs::{read_dir, ReadDir}, iter, path::Path};

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

fn trav() {

}

fn main() {
    // TODO: Take a directory as an argument to a function as an option.
    // Initially can just work from the directory it runs in.
    let Ok(current_directory) = Path::new(".").read_dir() else {
        panic!("Current directory is invalid"); 
    };

    // let b = Path::new(".");
    let _b = Path::new("../../../../cargo-projects/vivs/src/");
    let b = Path::new("./../../../cargo-projects/vivs/src");
    let path = env::current_dir().unwrap();

    println!("The current directory is {}", path.display());
    println!("{:?}", b);

    let r = b.read_dir().unwrap();

    for dir in current_directory {
        let Ok(dir_entry) = dir else {
            panic!("Directory entry is invalid");
        };

        let path = dir_entry.path();
        let last_entry = path.iter().last();

        let l = iter::repeat_n(" ", 3);
        let l: Vec<_> = l.collect();

        // let p = .to_str().unwrap();

        let Ok(current_dir) = env::current_dir() else {
            panic!("Current path incorrect");
        };

        let processed_entry = true;

        if path.is_dir() {
            // let w = format!("{}-- {}", l.join(""), p);
            println!("  {p}");
        } else {
            // let h = format!("{}-- {}", l.join(""), p);
            println!("{}", p);
        }

        // g.is_dir();
    }



    // traverse_file_tree(0, &a.unwrap());
}

