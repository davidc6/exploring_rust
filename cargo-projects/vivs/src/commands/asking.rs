use crate::{parser::Parser, ClusterInstanceConfig, Connection, GenericResult};
use core::str;
use std::env::current_dir;
use tokio::fs;

pub const ASKING_CMD: &str = "asking";

pub async fn check_ask(key: &str, conn: &mut Connection) -> Option<(u16, String)> {
    // check if config exists, we'll most likely need to store it in memory to avoid constant IO (?)
    let own_addr = conn.own_addr().unwrap().to_string();

    let port = own_addr.split(":").nth(1).unwrap_or_default();
    let current_dir = current_dir().unwrap();
    let node_config = format!("{}/{}.toml", current_dir.display(), port);

    // When no cluster (<port>.conf) file is found,
    // We can assume that Vivs is not running in the cluster mode.
    // Therefore normal processing of incoming command should take place.
    let Ok(file_contents) = fs::read(node_config).await else {
        return None;
    };

    let file_contents = str::from_utf8(&file_contents[0..]).unwrap();
    let nodes = toml::from_str::<ClusterInstanceConfig>(file_contents).unwrap();

    // Work out a cell / hash slot
    const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
    let key_hash = X25.checksum(key.as_bytes()) % 16384;

    // Iterate over all current nodes in the cluster
    for (ip, config) in nodes {
        let cell_range = config.position.0..config.position.1;

        let is_in_range = cell_range.contains(&key_hash.into());
        if own_addr == ip && is_in_range {
            return None;
        }

        if is_in_range {
            return Some((key_hash, ip));
        }
    }

    None
}

/// ASKING command is sent to the target node followed by the redirection command.
pub trait AskCommand {
    async fn check_ask(&self, key: &str, conn: &mut Connection) -> Option<(u16, String)> {
        // check if config exists, we'll most likely need to store it in memory to avoid constant IO (?)
        let own_addr = conn.own_addr().unwrap().to_string();

        let port = own_addr.split(":").nth(1).unwrap_or_default();
        let current_dir = current_dir().unwrap();
        let node_config = format!("{}/{}.toml", current_dir.display(), port);

        // When no cluster (<port>.conf) file is found,
        // We can assume that Vivs is not running in the cluster mode.
        // Therefore normal processing of incoming command should take place.
        let Ok(file_contents) = fs::read(node_config).await else {
            return None;
        };

        let file_contents = str::from_utf8(&file_contents[0..]).unwrap();
        let nodes = toml::from_str::<ClusterInstanceConfig>(file_contents).unwrap();

        // Work out a cell / hash slot
        const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
        let key_hash = X25.checksum(key.as_bytes()) % 16384;

        // Iterate over all current nodes in the cluster
        for (ip, config) in nodes {
            let cell_range = config.position.0..config.position.1;

            let is_in_range = cell_range.contains(&key_hash.into());
            if own_addr == ip && is_in_range {
                return None;
            }

            if is_in_range {
                return Some((key_hash, ip));
            }
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct Asking {
    command: String,
    slot: String,
    address: String,
}

impl Asking {
    pub fn new(command: String, slot: String, address: String) -> Self {
        Asking {
            command,
            slot,
            address,
        }
    }

    pub fn parse(mut data: Parser) -> Self {
        // Get the command first (i.e. GET)
        let Ok(command) = data.next_as_str() else {
            return Self::default();
        };

        // Get the slot second (i.e. 2345)
        let Ok(slot) = data.next_as_str() else {
            return Self::default();
        };

        // Get the address third (i.e. 127.0.0.1)
        let Ok(address) = data.next_as_str() else {
            return Self::default();
        };

        Self {
            command: command.unwrap(),
            slot: slot.unwrap(),
            address: address.unwrap(),
        }
    }

    pub fn get(&self) -> Option<String> {
        Some(format!(
            "{} {} {} {}",
            ASKING_CMD.to_lowercase(),
            self.command,
            self.slot,
            self.address
        ))
    }
}

impl Asking {
    pub async fn respond(self, conn: &mut Connection) -> GenericResult<()> {
        let length_of_address = self.slot.len();
        let cmd = self.command.to_uppercase();
        let cmd_len = cmd.len();

        let cmd = format!(
            "*2\r\n${cmd_len}\r\n{cmd}\r\n${length_of_address}\r\n{}\r\n",
            self.slot
        );

        let _ = conn.write_complete_frame(&cmd).await;

        Ok(())
    }

    // / Pushes optional PING [message] to the segments array if it exists.
    // / In order to do this, a default Parser gets created which
    // / takes is a command first and then the optional message.
    // / This is a bit of a hack since Parser and DataChunk are
    // / different structs (even though potentially get could be one in the future).
    // pub fn into_chunk(self) {
    // let data_chunk_frame = Parser::default();
    // let cmd = format!("{}\r\n", PING_CMD);
    // let mut data_chunk_frame = data_chunk_frame.push_bulk_str(Bytes::from(cmd));

    // if let Some(msg) = self.message {
    //     data_chunk_frame = data_chunk_frame.push_bulk_str(format!("{}\r\n", msg).into());
    // }

    //         // data_chunk_frame
    //     }
}
