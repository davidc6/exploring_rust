pub fn format_err_msg(str: String) -> String {
    format!("ERR {}\r\n", str)
}

pub fn num_args_err() -> &'static str {
    "Incorrect number of arguments\r\n"
}
