use bytes::Bytes;
use std::io::{stdin, stdout, Write};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use vivs::commands::ping::PONG;
use vivs::parser::Parser;
use vivs::{data_chunk::DataChunk, Connection, GenericResult, PORT};

pub async fn write_complete_frame(stream: &mut TcpStream, data: &str) -> std::io::Result<()> {
    stream.write_all(data.as_bytes()).await?;
    stream.flush().await
}

pub async fn read_chunk_frame(data_chunk: &mut Parser) -> GenericResult<Bytes> {
    match data_chunk.next() {
        Some(DataChunk::Bulk(data_bytes)) => {
            // This is a hack in order to write consistently formatted values to stdout.
            // Since val without quotes can also be written back to stdout without quotes
            // it is not desirable and therefore we want to add extra quotes to the output value.
            // We need to think about allocations here as it will affect performance in the long run.
            // 34 is "
            if data_bytes.first() != Some(&34) && data_bytes != *PONG {
                let quotes_bytes = Bytes::from("\"");
                let concat_bytes = [quotes_bytes.clone(), data_bytes, quotes_bytes].concat();
                Ok(Bytes::from(concat_bytes))
            } else {
                Ok(data_bytes)
            }
        }
        Some(DataChunk::Null) => Ok(Bytes::from("(nil)")),
        Some(DataChunk::SimpleError(data_bytes)) => Ok(data_bytes),
        Some(DataChunk::Integer(val)) => {
            // convert Bytes to bytes array
            // then determine endianness to create u64 integer value from the bytes array
            // and return integer as string
            let bytes_slice = val.slice(0..8);

            // converts the slice to an array of u8 elements (since u64 is 8 bytes)
            let arr_u8: [u8; 8] = bytes_slice[0..8].try_into().unwrap();
            let integer_as_string = if cfg!(target_endian = "big") {
                u64::from_be_bytes(arr_u8)
            } else {
                u64::from_le_bytes(arr_u8)
            }
            .to_string();

            Ok(Bytes::from(format!("(integer) {}", integer_as_string)))
        }
        None => Ok(Bytes::from("Unknown")),
        _ => Ok(Bytes::from("(nil)")), // catch all case
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    let address = format!("127.0.0.1:{}", PORT);
    let stream = TcpStream::connect(address).await?;
    let mut connection = Connection::new(stream);

    loop {
        // write to stdout
        write!(stdout(), "> ")?;
        // flush everything, ensuring all content reach destination (stdout)
        stdout().flush()?;

        // buffer for stdin's line of input
        let mut buffer = String::new();
        // Read a line of input and append to the buffer.
        // stdin() is a handle in this case to the standard input of the current process
        // which gets "locked" and waits for newline or the "Enter" key (or 0xA byte) to be pressed.
        stdin().read_line(&mut buffer)?;

        let data_chunk_frame_as_str = DataChunk::from_string(&buffer);

        // writes bytes to server socket
        // e.g. *0\r\n$4\r\nPING\r\n$4\r\nMary\r\n
        connection
            .write_complete_frame(&data_chunk_frame_as_str)
            .await?;

        let mut buffer = connection.process_stream().await?;
        let data_chunk = DataChunk::read_chunk(&mut buffer).unwrap();
        let mut parser = Parser::new(data_chunk).unwrap();
        let bytes_read = DataChunk::read_chunk_frame(&mut parser).await.unwrap();

        stdout().write_all(&bytes_read)?;
        stdout().write_all(b"\r\n")?;
        stdout().flush()?;
    }
}
