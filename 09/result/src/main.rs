#![allow(unused_variables)]

use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
