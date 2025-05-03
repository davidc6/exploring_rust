use crate::{parser::Parser, Connection, GenericResult};
use core::str;

pub const ASKING_CMD: &str = "asking";

/// ASKING command is sent to the target node followed by the redirection command.
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
}
