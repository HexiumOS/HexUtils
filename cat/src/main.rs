use sap::{Argument, Parser};
use std::fs::File;
use std::io::prelude::*;

fn process_file(file_path: &str) -> std::io::Result<()> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Value(val) => {
                let _ = process_file(&val);
            }
            _ => {}
        }
    }
}
