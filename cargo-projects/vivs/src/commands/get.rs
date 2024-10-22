use super::{AskCommand, CommonCommand};
use crate::parser::Parser;
use crate::utils::INCORRECT_ARGS_ERR;
use crate::{ClusterInstanceConfig, Connection, DataStore, GenericResult};
use core::str;
use log::info;
use std::env::current_dir;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs;

pub const GET_CMD: &str = "get";

#[derive(Debug)]
pub struct Get {
    pub key: Option<String>,
}

impl AskCommand for Get {
    async fn check_ask(&self, conn: &mut Connection) -> Option<(u16, String)> {
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
        let key_hash = X25.checksum(self.key.as_ref().unwrap().as_bytes()) % 16384;

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

impl CommonCommand for Get {
    fn parse(mut data: Parser) -> Self {
        let Ok(key) = data.next_as_str() else {
            return Self { key: None };
        };

        Self { key }
    }

    async fn respond(&self, conn: &mut Connection, db: &DataStore) -> GenericResult<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(INCORRECT_ARGS_ERR.as_bytes()).await?;
            return Ok(());
        };

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                GET_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        if let Some(redirect_addr) = self.check_ask(conn).await {
            conn.write_chunk(
                super::DataType::SimpleError,
                Some(format!("ASK {} {}", redirect_addr.0, redirect_addr.1).as_bytes()),
            )
            .await?
        }

        let mut db_guard = db.db.write().await;

        if let Some(value) = db_guard.get(key) {
            let mut expiries_guard = db.expirations.write().await;

            // If key exists in cache then check for TTL and whether the value has expired.
            // If key exists but expired (i.e. current time is more than expiry time),
            // evict from both stores and return null.
            if let Some(unix_time) = expiries_guard.get(key) {
                let duration_now_s = SystemTime::now().duration_since(UNIX_EPOCH)?;

                if Duration::from_secs(*unix_time) <= duration_now_s {
                    db_guard.remove(key);
                    expiries_guard.remove(key);

                    conn.write_null().await?
                } else {
                    conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                        .await?
                }
            } else {
                conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                    .await?
            }
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
