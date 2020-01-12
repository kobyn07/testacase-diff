use std::fs::File;
use std::io;
use std::io::Read;


pub mod default;
pub mod simple;

pub fn get_file(path: &str) -> Result<Box<String>, io::Error> {
    let mut ret = String::new();
    File::open(path)?.read_to_string(&mut ret)?;
    ret.retain(|c| c != '\r');
    Ok(Box::new(ret))
}

