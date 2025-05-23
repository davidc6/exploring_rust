use crate::{ClusterInstanceConfig, Connection, GenericResult};
use core::str;
use std::env::current_dir;
use tokio::fs;

pub const ASK_CMD: &str = "ask";

pub struct AskResponse {
    pub key_hash: u16,
    pub ip: String,
}

/// Checks whether data is on the node that the client is connected to or
/// is on a different node that the system needs to provide "coordinates" for.
pub async fn check_ask(key: &str, conn: &mut Connection) -> Option<AskResponse> {
    // check if config exists, we'll most likely need to store it in memory to avoid constant IO (?)
    let own_addr = conn.own_addr().unwrap().to_string();

    let port = own_addr.split(":").nth(1).unwrap_or_default();
    let current_dir = current_dir().unwrap();
    let node_config = format!("{}/{}.toml", current_dir.display(), port);

    // When no cluster (<port>.toml) file is found,
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

    // Iterate over all current node configs in the cluster
    for (ip, config) in nodes {
        // I.e. range of cells / location in a single node
        let cell_range = config.position.0..config.position.1;

        let is_in_range = cell_range.contains(&key_hash.into());
        if own_addr == ip && is_in_range {
            return None;
        }

        if is_in_range {
            return Some(AskResponse { key_hash, ip });
        }
    }

    None
}

/// ASK command indicates that the key is temporarily being handled by a different node.
/// Only the next query will be send to the specified node.
///
/// This command is different to the MOVED command which indicates that the hash slot is
/// permanently served by a different node. Next queries will be ran against the specified node.
///
/// For example, -ASK 7162 127.0.0.1:9001 (-<error> <hash_slot> <ip:port>)
#[derive(Debug, Default)]
pub struct Ask {}

impl Ask {
    pub fn parse() -> Self {
        Self {}
    }
}

impl Ask {
    pub async fn respond(self, conn: &mut Connection) -> GenericResult<()> {
        conn.write_chunk(super::DataType::SimpleString, "-> Redirected".as_bytes())
            .await?;

        Ok(())
    }
}
