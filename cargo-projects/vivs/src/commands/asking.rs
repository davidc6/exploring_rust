use core::str;
use std::env::current_dir;

use crate::{commands::get::Get, parser::Parser, ClusterInstanceConfig, Connection, GenericResult};
use bytes::Bytes;
use log::info;
use tokio::fs;

use super::Command;

pub const ASK_CMD: &str = "ask";
pub const ASK: &str = "ASK";

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

// #[derive(Debug, Default)]
// pub struct Ask {
//     pub command: Box<Command>,
// }

#[derive(Debug, Default)]
pub struct Ask {}

impl Ask {
    pub fn parse() -> Self {
        Self {}
    }
}

impl Ask {
    pub async fn respond(self, conn: &mut Connection) -> GenericResult<()> {
        //     const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
        //     let value = *self.command;
        //     let k = match value {
        //         Command::Get(val) => val.key,
        //         _ => Some("".to_owned()),
        //     };

        //     let a = X25.checksum(k.unwrap().as_bytes()) % 16384;

        // We need a list nodes
        // Ranges per each node
        // and then where to redirect to

        // println!("SLOT {}", a); // 759

        // -ASK <SLOT> <address_to_reach_out>

        // if let Some(message) = self.message {
        //     info!(
        //         "{}",
        //         format!(
        //             "{:?} {:?} {:?}",
        //             conn.connected_peer_addr(),
        //             PING_CMD.to_uppercase(),
        //             message
        //         )
        //     );
        //     conn.write_chunk(super::DataType::SimpleString, Some(message.as_bytes()))
        //         .await?;
        // } else {
        //     info!(
        //         "{:?} {:?}",
        //         conn.connected_peer_addr(),
        //         PING_CMD.to_uppercase()
        //     );
        //     conn.write_chunk(super::DataType::SimpleString, Some(b"PONG"))
        //         .await?;
        // }

        conn.write_chunk(super::DataType::SimpleString, "-> Redirected".as_bytes())
            .await?;

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
