use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt}};

#[derive(Debug)]
enum DataTypes {
    Array
}

// carriage return 
fn parse_until_crlf(stream: &[u8]) -> Result<(&[u8], &[u8]), String> {
    // println!("{:?}", stream);
    for (index, val) in stream.iter().enumerate() {
        if val == &b'\r' && stream[index + 1] == b'\n' {
            // result and rest
            return Ok((&stream[..index], &stream[index + 2..]));
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

    // for (index, val) in buffer.iter().enumerate() {
    //     if val == &b'\r' && buffer[index + 1] == b'\n' {
    //         // result and rest
    //         return Ok((&buffer[..index], &buffer[index + 2..]));
    //     }
    // }

    println!("STR {:?} {:?}", str_size, left);

    let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(str_size).unwrap().parse::<u64>().unwrap() as usize);

    for val in left.iter().enumerate() {
        let pos = str_size[0] as char;
        println!("START {}", pos.to_digit(10).unwrap());
        let res = parse(&left[pos.to_digit(10).unwrap() as usize..]);
    }


    // let mut v = Vec::new();
    
    // for item in left {
    //     v.push(item);
    // }

    Ok((vec, left))
}

fn parse_array(buffer: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ()> {
    let array = parse_until_crlf(buffer);

    let arr_size = array.as_ref().unwrap().0;
    let left = array.as_ref().unwrap().1;
    
    // println!("ARR {:?}", arr_size);

    let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(arr_size).unwrap().parse::<u64>().unwrap() as usize);

    // vec.push(b"ds");
    for val in left.iter().enumerate() {
        let res = parse(left);
    }

    Ok((vec, left))

    // let mut arr_size = Vec::new();

    // for (index, char) in stream.iter().enumerate() {
    //     println!("{} {}", index, *char as char);

    //     if index == 0 {
    //         // arr_size.push(*char as char);
    //         let mut it = stream.iter();
    //         let mut next = it.next();

    //         while next != Some(&13) {
    //             println!("HERE {}", *next.unwrap() as char);
    //             arr_size.push(*next.unwrap() as char);
    //             next = it.next();
    //         }
    //         continue;
    //     }
    // }

    // println!("Size {:?}", arr_size.iter().collect::<String>());
    // return arr_size.iter().collect::<String>();
}

fn parse(buffer: &[u8]) {
    print!("Buffer {:?}\n", buffer);

    if let Some(data_type) = buffer.first() {
        let data_type = match data_type {
            b'*' => parse_array(&buffer[1..buffer.len()]),
            b'$' => parse_bulk_strings(&buffer[1..buffer.len()]),
                // let arr_let = &buffer[0..2];
                // println!("{:?}", arr_let);
            _ => Ok((Vec::with_capacity(1), buffer))
        };

        // println!("{:?}", data_type);

        

    } else {
        panic!("Unexpected");
    }
}

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let (read, write) = stream.split();
    let mut reader = BufReader::new(read);

    let mut buffer: Vec<u8> = vec![];
    reader.read_to_end(&mut buffer).await.expect("Error");
    let slice = &buffer[0..buffer.len()];

    parse(slice);

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
