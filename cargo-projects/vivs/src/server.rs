use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt, AsyncBufReadExt}};

#[derive(Debug)]
enum DataTypes {
    Array
}

// carriage return 
fn parse_until_crlf(stream: &[u8]) -> Result<(&[u8], &[u8]), String> {
    for (index, val) in stream.iter().enumerate() {
        if val == &b'\r' && stream[index + 1] == b'\n' {
            // result and rest
            return Ok((&stream[..index], &stream[index + 2..stream.len()]));
        }
    }

    // error
    return Err("Err".to_owned());
}

fn parse_bulk_strings(buffer: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ()> {
    // 4\r\nPING\r\n

    let array = parse_until_crlf(buffer);

    let str_size = array.as_ref().unwrap().0;
    let left = array.as_ref().unwrap().1;

    let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(str_size).unwrap().parse::<u64>().unwrap() as usize);

    let pos = str_size[0] as char;
    let res = parse(&left[pos.to_digit(10).unwrap() as usize..]);

    Ok((vec, left))
}

fn parse_array(buffer: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ()> {
    let array = parse_until_crlf(buffer);

    let arr_size = array.as_ref().unwrap().0;
    let left = array.as_ref().unwrap().1;
    
    let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(arr_size).unwrap().parse::<u64>().unwrap() as usize);

    let res = parse(left);

    Ok((vec, left))
}

struct BufSplit(usize, usize);

enum PartBuf {
    String(BufSplit),
    Array(Vec<BufSplit>)
}

enum ParseError {

}

type ParseResult = Result<Option<(usize, PartBuf)>, ParseError>;

fn parse(buffer: &[u8]) -> Result<(), ()> {
    println!("Buffer {:?}\n", buffer);

    if let Some(data_type) = buffer.first() {
        let data_type = match data_type {
            b'*' => parse_array(&buffer[1..buffer.len()]),
            b'$' => parse_bulk_strings(&buffer[1..buffer.len()]),
            b'\r' => Ok((Vec::with_capacity(1), buffer)),
            _ => Ok((Vec::with_capacity(1), buffer))
        };

        return Ok(());
    } else {
        // run the command
        // panic!("Unexpected");
        return Ok(());
    }

    return Ok(());
}

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) -> std::io::Result<()> {
    let (read, write) = stream.split();
    let reader = BufReader::new(read);
    let mut lines = reader.lines();

    loop {
        if let Ok(line) = lines.next_line().await {
            parse(line.unwrap().as_bytes());
            continue;
        }
        break;
    }

    Ok(())
}

pub async fn start(addr: String, port: String) -> std::io::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?; // TODO - checks?

        tokio::spawn(async move {
            handle_stream(stream, addr).await;
        });
    }
}
