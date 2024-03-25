pub fn unknown_cmd_err(value: String) -> String {
    format!("Unknown command: {value}\r\n")
}

pub const NO_CMD_ERR: &str = "No command supplied\r\n";
pub const INCORRECT_ARGS_ERR: &str = "Incorrect number of arguments\r\n";
pub const VALUE_NOT_INT_ERR: &str = "Value is not an integer\r\n";

pub fn u64_as_bytes(integer: u64) -> [u8; 8] {
    if cfg!(target_endian = "big") {
        integer.to_be_bytes()
    } else {
        integer.to_le_bytes()
    }
}
