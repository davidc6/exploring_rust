use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use tokio::net::TcpStream;
use vivs::Result;
use vivs::{commands::ping::Ping, Connection};

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
async fn main() -> Result<()> {
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

        let command = buffer.trim().to_owned();
        let mut line: std::str::Split<'_, char> = command.split(' ');

        let Some(cmd) = line.next() else {
            continue;
        };
        if cmd.is_empty() {
            continue;
        }

        // TODO: implement command parser
        let data_chunk = match cmd.to_lowercase().as_ref() {
            "ping" => Ping::new(line.next().map(|val| val.to_owned())).into_chunk(),
            _ => todo!(),
        };

        // convert to the byte stream
        // i.e. [Bulk("PING"), Bulk("Mary")]
        // *0\r\n$4\r\nPING\r\n$4\r\nMary\r\n

        // writes bytes to server socket
        // e.g. *0\r\n$4\r\nPING\r\n$4\r\nMary\r\n
        connection.write_chunk_frame(data_chunk).await?;

        // reads bytes from server socket
        // e.g. Mary
        let bytes_read = connection.read_chunk_frame().await?;

        stdout().write_all(&bytes_read)?;
        stdout().write_all(b"\n")?;
        stdout().flush()?;
    }
}
