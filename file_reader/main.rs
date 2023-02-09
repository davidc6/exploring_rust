#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }

    fn new_with_data(filename: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(filename);
        file.data = data.clone(); // clones pointer to data on the heap
        file
    }

    fn read(self: &File, buf: &mut Vec<u8>) -> usize {
        let mut temporary = self.data.clone(); // clone data
        let size = temporary.len(); // get size in order to create new vec

        buf.reserve(size); // allocate minimum capacity for additional elements (might alloc more space to avoid reallocations)
        buf.append(&mut temporary);

        size
    }
}

fn open_file(file: File) -> Result<File, ()> {
    Ok(file)
}

fn close_file(file: File) -> Result<File, ()> {
    Ok(file)
}

// global error
static mut ERROR: i32 = 0;

fn main() {
    let v: Vec<u8> = vec![104, 101, 108, 108, 111]; // decimal to character == hello
    let mut file = File::new_with_data("new-file.txt", &v);

    let mut buf: Vec<u8> = vec![];

    file = open_file(file).unwrap();
    let buf_len = file.read(&mut buf);
    file = close_file(file).unwrap();

    let s = String::from_utf8_lossy(&buf);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, buf_len);
    println!("{}", s);
}
