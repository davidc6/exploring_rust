pub fn unknown_cmd_err(value: String) -> String {
    format!("Unknown command: {value}\r\n")
}

pub const NO_CMD_ERR: &str = "No command supplied\r\n";
pub const INCORRECT_ARGS_ERR: &str = "Incorrect number of arguments\r\n";
pub const VALUE_NOT_INT_ERR: &str = "Value is not an integer\r\n";
