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

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }

    fn read(self, buf: &mut Vec<u8>) -> usize {
        let mut temporary = self.data.clone(); // clone data
        let size = self.data.len(); // get size in order to create new vec

        buf.reserve(size); // allocate minimum capacity for additional elements (might more space to avoid reallocations)
        buf.append(&mut temporary);

        size
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let file = File::new_with_data("new-file", v);

    let mut buf = vec![];

    file.read(&mut buf);

    let s = String::from_utf8_lossy(&buf);
    println!("{:?}", s);
}
