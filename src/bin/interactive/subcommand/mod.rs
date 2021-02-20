pub mod add;

use std::io;
use std::io::{Read, stdout, Write};

pub(crate) fn require_string(req: &str, require_msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut str = ask_for_string(req, "")?;
    while str.is_empty() {
        println!("{}", require_msg);
        str = ask_for_string(req, "")?;
    }

    Ok(str)
}

pub(crate) fn ask_for_string(req: &str, default: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut input_req = req.to_string();
    if !default.is_empty() {
        input_req.push_str(&format!(" ({})", default));
    }

    print!("{}: ", input_req);
    let _ = stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    if let Some('\n')=buffer.chars().next_back() {
        buffer.pop();
    }
    if let Some('\r')=buffer.chars().next_back() {
        buffer.pop();
    }

    if buffer.is_empty() { Ok(default.to_string()) }
    else { Ok(buffer) }
}
