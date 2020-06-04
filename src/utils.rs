use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &str) -> Result<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
