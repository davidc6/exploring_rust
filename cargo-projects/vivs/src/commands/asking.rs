use crate::{parser::Parser, Connection, GenericResult};
use core::str;

pub const ASKING_CMD: &str = "asking";

/// ASKING command is sent to the target node followed by the redirection command.
#[derive(Debug, Default)]
pub struct Asking {
    command: String,
    key: String,
    address: String,
}

impl Asking {
    pub fn new(command: String, key: String, address: String) -> Self {
        Self {
            command,
            key,
            address,
        }
    }

    pub fn command(mut self, command: String) -> Asking {
        self.command = command;
        self
    }

    pub fn key(mut self, key: String) -> Asking {
        self.key = key;
        self
    }

    pub fn address(mut self, address: String) -> Asking {
        self.address = address;
        self
    }

    pub fn build(self) -> Asking {
        Asking {
            command: self.command,
            key: self.key,
            address: self.address,
        }
    }

    pub fn parse(mut data: Parser) -> Self {
        // TODO - treat errors accordingly here (i.e. data.next_as_str()?)
        // First, get the command (i.e. GET)
        let Ok(command) = data.next_as_str() else {
            return Self::default();
        };

        // Second, get the slot (i.e. 2345)
        let Ok(slot) = data.next_as_str() else {
            return Self::default();
        };

        // Third, get the address (i.e. 127.0.0.1)
        let Ok(address) = data.next_as_str() else {
            return Self::default();
        };

        // It's OK to unwrap here, since we have checks above
        Self {
            command: command.unwrap(),
            key: slot.unwrap(),
            address: address.unwrap(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{ASKING_CMD} {} {} {}",
            self.command, self.key, self.address
        )
    }
}

impl Asking {
    pub async fn respond(self, conn: &mut Connection) -> GenericResult<()> {
        let length_of_address = self.key.len();
        let cmd = self.command.to_uppercase();
        let cmd_len = cmd.len();

        // 2 is hard-coded here
        let cmd = format!(
            "*2\r\n${cmd_len}\r\n{cmd}\r\n${length_of_address}\r\n{}\r\n",
            self.key
        );

        println!("COMMAND {:?}", cmd);

        let _ = conn.write_complete_frame(&cmd).await;

        Ok(())
    }
}
