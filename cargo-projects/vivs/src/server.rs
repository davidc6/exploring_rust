use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt}};

#[derive(Debug)]
enum DataTypes {
    Array
}

fn parse_array(stream: &[u8]) -> String {
    let mut arr_size = Vec::new();

    for (index, char) in stream.iter().enumerate() {
        println!("{} {}", index, *char as char);

        if index == 0 {
            // arr_size.push(*char as char);
            let mut it = stream.iter();
            let mut next = it.next();

            while next != Some(&13) {
                println!("HERE {}", *next.unwrap() as char);
                arr_size.push(*next.unwrap() as char);
                next = it.next();
            }
            continue;
        }
    }

    println!("Size {:?}", arr_size.iter().collect::<String>());
    return arr_size.iter().collect::<String>();
}

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let (read, write) = stream.split();
    let mut reader = BufReader::new(read);

    let mut buffer: Vec<u8> = vec![];
    reader.read_to_end(&mut buffer).await.expect("Error");

    if let Some(data_type) = buffer.first() {
        let data_type = match *data_type as char {
            '*' => {

                parse_array(&buffer[1..]);

                // let arr_let = &buffer[0..2];
                // println!("{:?}", arr_let);
            },
            _ => todo!()
        };

        // println!("{:?}", data_type);

        

    } else {
        panic!("Unexpected");
    }



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
