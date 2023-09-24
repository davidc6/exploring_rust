use std::{
    io::{stdin, stdout, Write},
    net::TcpStream,
};

use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    // REPL
    loop {
        // write to stdout
        write!(stdout(), "> ")?;
        // flush everything, ensuring all content reach destination (stdout)
        stdout().flush()?;

        let mut buffer = String::new();
        // Read a line of input and append to the buffer.
        // stdin() is a handle in this case to the standard input of the current process
        // which gets locked and waits for newline or the "Enter" key (or 0xA byte) to be pressed.
        stdin().read_line(&mut buffer)?;

        let command = buffer.trim_end().to_lowercase().to_owned();

        let buffer = match command.as_ref() {
            "ping" => format!(
                "{}\r\n{}{}\r\n{}\r\n",
                "*1",                         // number of elements
                "\x24",                       // $
                command.len(),                // <number_of_chars>
                buffer.trim_end().to_owned(), // command (e.g. PING)
            ),
            &_ => unimplemented!(),
        };

        stream.write_all(buffer.as_bytes())?;
        stream.flush()?;

        let mut buffer = [0; 32];
        let read_amount = stream.read(&mut buffer)?;

        // if read is 0 then socket is most probably closed
        // first we write the response and then the \n newline character
        stdout().write_all(
            format!("{:?}", std::str::from_utf8(&buffer[..read_amount]).unwrap()).as_bytes(),
        )?;
        stdout().write_all(b"\n")?;
        stdout().flush()?;
    }
}
