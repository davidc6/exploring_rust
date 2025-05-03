use clap::{Args, Parser as ClapParser, Subcommand};
use env_logger::Env;
use log::info;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::io::{stdin, stdout, Cursor, Write};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use vivs::commands::ask::ASK_CMD;
use vivs::commands::asking::Asking;
use vivs::commands::get::GET_CMD;
use vivs::commands::ping::PONG;
use vivs::parser::Parser;
use vivs::ClusterConfig;
use vivs::{data_chunk::DataChunk, Connection, GenericResult};

pub async fn write_complete_frame(stream: &mut TcpStream, data: &str) -> std::io::Result<()> {
    stream.write_all(data.as_bytes()).await?;
    stream.flush().await
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
    Create { ip_addresses: Vec<String> },
}

#[derive(Debug, ClapParser)]
struct Cli {
    #[arg(long)]
    host: Option<String>,
    #[arg(long, short)]
    port: Option<u16>,
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

async fn set_up_cluster(cli_args: Cli) -> GenericResult<()> {
    info!("Enabling cluster mode");

    let mut active_instances = HashSet::new();
    let mut instances: HashMap<String, ClusterConfig> = HashMap::new();
    let mut current_config = HashMap::new();

    // Create cluster command
    if let Some(Commands::Create { ip_addresses }) = cli_args.command {
        let total_ips = ip_addresses.len();

        // There are 16384 cells
        const CELLS_TOTAL: usize = 16384;
        let slice = CELLS_TOTAL / total_ips;

        let mut start = 0;
        let mut end = slice - 1;

        // To enable us to access ips by index later
        for i in 0..total_ips {
            let mut cur: usize = 0;

            let Some(current_ip) = ip_addresses.get(i) else {
                continue;
            };

            let Some(current_port) = current_ip.split(":").nth(1) else {
                info!("{current_ip} IP does not contain a port");
                continue;
            };

            // We need this to write the config file
            let mut is_current_port_open = false;

            // Iterate over the passed in ips
            for ip_address in &ip_addresses {
                // Can a TCP connection to the node be established?
                let stream = TcpStream::connect(ip_address).await;

                let Ok(_) = stream else {
                    info!("Could not connect to {ip_address}");
                    continue;
                };

                let ping = DataChunk::from_string("PING");

                // send PING command to <ip:port>
                let mut conn = Connection::new(stream.unwrap());
                conn.write_complete_frame(&ping).await?;

                // process response
                let mut buffer = conn.process_stream().await?;
                let mut parser = Parser::new(DataChunk::read_chunk(&mut buffer)?)?;
                let bytes_read = DataChunk::read_chunk_frame(&mut parser).await?;

                // We know that a Vivs instance is running if we PING it and it PONGs back
                if bytes_read == PONG.as_bytes() {
                    // We are connected to the Vivs instance
                    if current_ip == ip_address {
                        is_current_port_open = true;
                    }

                    active_instances.insert(ip_address.clone());

                    //
                    if let Some(node) = instances.get_mut(ip_address) {
                        node.is_self = i == cur;

                        current_config.insert(ip_address.clone(), node.clone());
                        cur += 1;

                        continue;
                    }

                    let node_id = rand_number_as_string().await?;

                    let mut config = ClusterConfig {
                        id: node_id.clone(),
                        ip: ip_address.clone(),
                        is_self: false,
                        position: (start, end),
                    };
                    if i == cur {
                        config.is_self = true;
                    }
                    cur += 1;

                    instances.insert(ip_address.clone(), config.clone());
                    current_config.insert(ip_address.clone(), config);
                }

                start += slice + 1;

                if slice * 2 > CELLS_TOTAL - 1 {
                    end = CELLS_TOTAL - 1;
                } else {
                    end += slice * 2;
                }
            }

            // Vivs instance node is active and listening
            if is_current_port_open {
                let path = current_dir();

                let mut file =
                    File::create(format!("{}/{}.toml", path.unwrap().display(), current_port))
                        .await?;

                let toml_as_string = toml::to_string(&current_config).unwrap();
                file.write_all(toml_as_string.as_bytes()).await?;
            }
        }
    }

    if active_instances.is_empty() {
        Err("Could not create a cluster mode since no Vivs instances are running")?;
    }

    Ok(())
}

async fn parse_stream(connection: &mut Connection) -> GenericResult<Parser> {
    let mut buffer = connection.process_stream().await?;
    let data_chunk = DataChunk::read_chunk(&mut buffer)?;
    let parser = Parser::new(data_chunk)?;
    Ok(parser)
}

fn write_to_stdout(bytes: &[u8]) -> GenericResult<()> {
    stdout().write_all(bytes)?;
    stdout().write_all(b"\r\n")?;
    stdout().flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = Cli::parse();

    // TODO: check args order
    if cli_args.cluster {
        return set_up_cluster(cli_args).await;
    }

    let node_port = cli_args.port.unwrap_or(9000);
    let node_host = cli_args.host.unwrap_or("127.0.0.1".to_owned());

    let mut address = format!("{node_host}:{node_port}");
    let stream = TcpStream::connect(address.clone()).await?;
    let mut connection = Connection::new(stream);
    let mut other_addr = "".to_string();

    // A command that needs to be processed
    let mut command_to_process: Option<Cursor<&[u8]>> = None;
    // This is in case we need to store the initial/previous command
    // in situations like GET -> ASK
    let mut initial_command: Vec<String> = vec![];

    loop {
        // Peek at the command to see if there's anything to process
        if let Some(ref mut cursor) = command_to_process {
            let command_as_data_chunk = DataChunk::read_chunk(cursor).unwrap();
            // This parser contains the command itself i.e. GET A or ASK <slot> <address>
            let mut parser = Parser::new(command_as_data_chunk)?;

            // The response from the server is Null (TODO: update implementation)
            let Some(command) = parser.peek_as_str() else {
                let bytes_read: bytes::Bytes = DataChunk::read_chunk_frame(&mut parser).await?;
                command_to_process = None;

                write_to_stdout(&bytes_read)?;
                continue;
            };

            // ASK command needs to be treated differently,
            // when a client receives ASK command it needs to send
            // ASKING to interact with a node that has the data
            if command.to_lowercase() == ASK_CMD.to_lowercase() {
                let _ = parser.next_as_str().unwrap().unwrap(); // current command i.e. ASK
                let slot = parser.next_as_str().unwrap().unwrap(); // a slot where the value is stored at
                let address = parser.next_as_str().unwrap().unwrap(); // new address to query for the value

                other_addr = address.clone();

                // ASKING GET <key> <address> - should be set to the target node instead
                let command_to_forward = initial_command.first().cloned().unwrap();
                // This is tied to GET command
                let key_to_forward = initial_command.get(1).unwrap().to_owned();
                // Ideally, we shouldn't need command + value, it should be one
                let command = Asking::default()
                    .command(command_to_forward)
                    .key(key_to_forward)
                    .address(address.clone())
                    .build()
                    .format();

                let command_parsed = DataChunk::from_string(&command);
                let mut command_as_bytes = Cursor::new(command_parsed.as_bytes());
                let command_as_data_chunk = DataChunk::read_chunk(&mut command_as_bytes).unwrap();
                let mut command_as_data_chunk = Parser::new(command_as_data_chunk)?;

                connection
                    .write_chunk_frame(&mut command_as_data_chunk)
                    .await?;

                let buffer = connection.process_stream().await?;
                command_to_process = Some(buffer);
                continue;
            }

            if command.to_lowercase() == GET_CMD.to_lowercase() {
                address = other_addr.clone();
                let stream = TcpStream::connect(address.clone()).await?;
                connection = Connection::new(stream);

                connection.write_chunk_frame(&mut parser).await?;

                let buffer = connection.process_stream().await?;
                command_to_process = Some(buffer);
                continue;
            }

            let bytes_read = DataChunk::read_chunk_frame(&mut parser).await?;
            command_to_process = None;

            write_to_stdout(&bytes_read)?;
        } else {
            // write to stdout
            write!(stdout(), "{address}> ")?;

            // flush everything, ensuring all content reach destination (stdout)
            stdout().flush()?;

            let mut buffer = String::new();
            // Read a line of input and append to the buffer.
            // stdin() is a handle in this case to the standard input of the current process
            // which gets "locked" and waits for newline or the "Enter" key (or 0xA byte) to be pressed.
            stdin().read_line(&mut buffer)?;

            initial_command = buffer
                .split_whitespace()
                .map(|val| val.to_string())
                .collect();

            let data_chunk_frame_as_str = DataChunk::from_string(&buffer);

            // write bytes to server socket
            // e.g. *0\r\n$4\r\nPING\r\n$4\r\nMary\r\n
            connection
                .write_complete_frame(&data_chunk_frame_as_str)
                .await?;

            let buffer = connection.process_stream().await?;
            command_to_process = Some(buffer);
        }
    }
}
