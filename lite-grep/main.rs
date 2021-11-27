use std::{ env, fs };
use std::path::PathBuf;
use std::io::Result;
use std::ffi::{ OsStr, OsString };

fn search_in_current_dir(current_dir: Result<PathBuf>, needle: &String) -> Result<Vec<OsString>> {
  let mut vec = Vec::new();
  let txt_file_ext = OsStr::new("txt");

  // ? - error propagation (sugar), if error occurs it will returned to the caller (i.e. main)
  for entry in fs::read_dir(current_dir.unwrap().as_path())? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
      let ext = path.extension();

      if ext.is_some() && ext == Some(txt_file_ext) {
        let contents = fs::read_to_string(&path)?;

        if contents.contains(needle) {
          match path.file_name() {
            Some(x) => vec.push(x.to_os_string()),
            None => eprintln!("Error")
          }
        }
      }
    }
  }

  Ok(vec)
}

fn main() {
  // 1. take in command line arg, .collect() allows us to transform into a collection
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    return println!("{}", "Please provide a \"needle\"");
  }

  let dir = env::current_dir();
  let matches = search_in_current_dir(dir, &args[1]);

  match matches {
    Ok(x) => println!("{:?}", x),
    Err(e) => eprintln!("Error: {}", e)
  }
}

