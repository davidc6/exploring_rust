pub fn format_err_msg(str: String) -> String {
    format!("ERR {}\r\n", str)
}

pub fn num_args_err() -> &'static str {
    "Incorrect number of arguments\r\n"
}

pub fn unknown_cmd_err(value: String) -> String {
    format!("Unknown command: {value}\r\n")
}
