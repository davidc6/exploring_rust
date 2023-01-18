use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt}};

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

fn parse(buffer: &[u8]) -> Result<(), ()> {
    print!("Buffer {:?}\n", buffer);

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
    let mut reader = BufReader::new(read);

    // let mut buffer: Vec<u8> = vec![];
    // reader.read_to_end(&mut buffer).await?;
    let mut buffer = [0; 14];
    reader.read_exact(&mut buffer).await?;

    let slice = &buffer[0..buffer.len()];

    parse(slice);

    match write.try_write(b"+PONG\r\n") {
        Ok(val) => {
            println!("Final read: {:?}", val);
        }
        Err(e) => {

        }
    };
    // return;

    // if let Some(data_type) = buffer.first() {
    //     let data_type = match *data_type as char {
    //         '*' => {

    //             parse_array(&buffer[1..]);

    //             // let arr_let = &buffer[0..2];
    //             // println!("{:?}", arr_let);
    //         },
    //         _ => todo!()
    //     };

    //     // println!("{:?}", data_type);

        

    // } else {
    //     panic!("Unexpected");
    // }



    // let mut buf = String::new();
    // let a = reader.read_to_string(&mut buf).await.expect("Err");
    // println!("{:?}", buf);


    // loop {
    //     let mut buffer = String::new();
    //     let bytes = reader.read_line(&mut buffer).await.expect("Error");
    

        
    //     for (size, char) in buffer.chars().enumerate() {
    //         println!("{}", char);
    //     }


    
    //     if bytes == 0 {
    //         println!("ZERO");
    //     }
    // }


    // loop {
    //     let mut buffer = String::new();
    //     let x = read_buffer.read_line(&mut buffer).await;

    //     match x {
    //         Ok(val) => {
    //             match write.try_write(b"+PONG\r\n") {
    //                 Ok(val) => {
    //                     println!("{:?}", buffer);
    //                 }
    //                 Err(e) => {

    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             println!("{}", e);
    //         }
    //     }

    //     buffer.clear();
    // }
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
