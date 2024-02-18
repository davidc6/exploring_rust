use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use tokio::net::TcpStream;
use vivs::{data_chunk::DataChunk, Connection, GenericResult};

#[derive(Debug)]
pub enum CliError {
    MissingCommand,
}

impl std::error::Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::MissingCommand => write!(f, "TCP connection closed"),
        }
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    let stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut connection = Connection::new(stream);

    // REPL
    loop {
        // let (reader, writer) = stream.into_split();
        // write to stdout
        write!(stdout(), "> ")?;
        // flush everything, ensuring all content reach destination (stdout)
        stdout().flush()?;

        // buffer for stdin's line of input
        let mut buffer = String::new();
        // Read a line of input and append to the buffer.
        // stdin() is a handle in this case to the standard input of the current process
        // which gets locked and waits for newline or the "Enter" key (or 0xA byte) to be pressed.
        stdin().read_line(&mut buffer)?;

        let data_chunk_frame_as_str = DataChunk::from_string(&buffer)?;

        // writes bytes to server socket
        // e.g. *0\r\n$4\r\nPING\r\n$4\r\nMary\r\n
        connection
            .write_complete_frame(&data_chunk_frame_as_str)
            .await?;

        // reads bytes from server socket
        let bytes_read = connection.read_chunk_frame().await?;

        // write back to stdout
        stdout().write_all(&bytes_read)?;
        stdout().write_all(b"\n")?;
        stdout().flush()?;
    }
}
