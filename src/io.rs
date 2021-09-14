use std::fs;
use std::io;
use std::io::{Read, Result};
use std::iter::Iterator;

// Writes output to text file
pub fn write_output(filename: &str, output: &str) -> Result<()> {
    fs::write(filename, output)?;
    Ok(())
}

// Reads file in root
pub fn read_input(filename: &str) -> Result<String> {
    let res = fs::read_to_string(filename)?;
    Ok(res)
}

pub fn read_stdin() -> Result<String> {
    let mut buf = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;
    let res: String = buf.iter().map(|&v| v as char).collect();
    Ok(res)
}
