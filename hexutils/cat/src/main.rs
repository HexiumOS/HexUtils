use sap::{Parser, Argument};
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
                let _ = process_file(&val); // Call your function
            },
            Argument::Long(opt) => println!("Error, file path expected, got long option: --{}", opt),
            Argument::Short(c) => println!("Error, file path expected, got short option: -{}", c),
            Argument::Stdio => println!("Error, file path expected, got stdin"),
        }
    }
}
