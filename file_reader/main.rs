use std::fmt;
use std::fmt::{Display};

trait Read {
    fn read(self: &Self, buffer: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &Self, buffer: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed
}

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    file_state: FileState
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            file_state: FileState::Closed
        }
    }

    fn new_with_data(filename: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(filename);
        file.data = data.clone(); // clones pointer to data on the heap
        file
    }

    fn read(self: &File, buf: &mut Vec<u8>) -> Result<usize, String> {
        if self.file_state != FileState::Open {
            return Err(String::from("This file should be open for reading"));
        }

        let mut temporary = self.data.clone(); // clone data
        let size = temporary.len(); // get size in order to create new vec

        buf.reserve(size); // allocate minimum capacity for additional elements (might alloc more space to avoid reallocations)
        buf.append(&mut temporary);

        Ok(size)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File: {} | State: {}", self.name, self.file_state)
    }
}

fn open_file(mut file: File) -> Result<File, ()> {
    file.file_state = FileState::Open;
    Ok(file)
}

fn close_file(mut file: File) -> Result<File, ()> {
    file.file_state = FileState::Closed;
    Ok(file)
}

// global error
static mut ERROR: i32 = 0;

fn main() {
    // reading with data
    // let v: Vec<u8> = vec![104, 101, 108, 108, 111]; // decimal to character == hello
    // let mut file = File::new_with_data("new-file.txt", &v);

    // let mut buf: Vec<u8> = vec![];

    // file = open_file(file).unwrap();
    // let buf_len = file.read(&mut buf).unwrap();
    // file = close_file(file).unwrap();

    // let s = String::from_utf8_lossy(&buf);

    // println!("{:?}", file);
    // println!("{} is {} bytes long", &file.name, buf_len);
    // println!("{}", s);

    // reading without data
    let mut buf: Vec<u8> = vec![];
    let mut f = File::new("new-file.txt");

    f = open_file(f).unwrap();

    if f.read(&mut buf).is_err() {
        // error
    }

    let buf_len = f.read(&mut buf).unwrap();
    f = close_file(f).unwrap();

    let s = String::from_utf8_lossy(&buf);

    println!("{}", f);
    println!("{:?}", f);
}
