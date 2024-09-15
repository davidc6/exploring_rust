use bytes::Bytes;
use clap::{Args, Parser as ClapParser, Subcommand};
use serde::Serialize;
use std::env::{args, current_dir};
use std::io::{stdin, stdout, Write};
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use toml::value;
use vivs::commands::ping::PONG;
use vivs::parser::Parser;
use vivs::{data_chunk::DataChunk, Connection, GenericResult};

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
            let arr_u8: Result<[u8; 8], _> = bytes_slice[0..8].try_into();
            let Ok(arr_u8) = arr_u8 else {
                return Ok(Bytes::from("(nil)"));
            };

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

#[derive(Args, Debug, Clone)]
struct ClusterCommands {
    #[arg(long)]
    create: Vec<String>,
    #[arg(long)]
    delete: Vec<String>,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Create { addresses: Vec<String> },
}

// #[derive(Subcommand)]
// struct OriginalCommands {}

#[derive(Debug, ClapParser)]
struct Cli {
    #[arg(long, short)]
    cluster: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

async fn rand_number_as_string() -> GenericResult<String> {
    let mut open_file = File::open("/dev/urandom").await?;

    let mut rand_integers = [0u8; 10];
    open_file.read_exact(&mut rand_integers).await?;

    Ok(rand_integers
        .to_vec()
        .iter()
        .map(|integer| format!("{:x}", integer))
        .collect::<Vec<_>>()
        .join(""))
}

#[derive(Serialize)]
struct Config {
    id: String,
    ip: String,
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    let cli_args = Cli::parse();

    // check order for
    if cli_args.cluster {
        println!("Cluster mode enabled: {:?}", cli_args);

        let node_id = rand_number_as_string().await?;

        let config = Config {
            id: node_id,
            ip: "127.0.0.1".to_owned(),
        };

        let toml_as_string = toml::to_string(&config).unwrap();

        let mut active_instances = vec![];

        if let Some(Commands::Create { addresses }) = cli_args.command {
            for address in addresses {
                let stream = TcpStream::connect(address.clone()).await;

                if stream.is_err() {
                    continue;
                }

                let ping = DataChunk::from_string("PING");

                // send PING command to <ip:port>
                let mut conn = Connection::new(stream.unwrap());
                conn.write_complete_frame(&ping).await?;

                // process response
                let mut buffer = conn.process_stream().await?;
                let data_chunk = DataChunk::read_chunk(&mut buffer)?;
                let mut parser = Parser::new(data_chunk)?;
                let bytes_read = DataChunk::read_chunk_frame(&mut parser).await?;

                // we know that a Vivs instance is running if we PING it and it PONGs back
                if bytes_read == PONG.as_bytes() {
                    active_instances.push(address.clone());

                    let port: Vec<_> = address.split(":").collect();
                    let path = current_dir();
                    let mut file = File::create(format!(
                        "{}/{}.toml",
                        path.unwrap().display(),
                        port.get(1).unwrap()
                    ))
                    .await?;
                    // let a = value(v);
                    file.write_all(toml_as_string.as_bytes()).await?;
                }
            }
        }

        if active_instances.is_empty() {
            Err("Could not create a cluster mode since no Vivs instances are running")?;
        }

        // redis-cli --cluster create 127.0.0.1:7000 127.0.0.1:7001 --cluster-replicas 1

        // - check address is active
        // - ping and pong

        return Ok(());
    }

    let address = format!("127.0.0.1:{}", 9000);
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
        let data_chunk = DataChunk::read_chunk(&mut buffer)?;
        let mut parser = Parser::new(data_chunk)?;
        let bytes_read = DataChunk::read_chunk_frame(&mut parser).await?;

        stdout().write_all(&bytes_read)?;
        stdout().write_all(b"\r\n")?;
        stdout().flush()?;
    }
}
