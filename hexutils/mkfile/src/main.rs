use sap::{Parser, Argument};
use std::fs::File;

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Value(val) => {
                make_file(&val); // Call your function
            },
            Argument::Long(opt) => println!("Error, name expected, got long option: --{}", opt),
            Argument::Short(c) => println!("Error, name expected, got short option: -{}", c),
            Argument::Stdio => println!("Error, name expected, got stdin"),
        }
    }
}

fn make_file(file_name: &str) {
    File::create(file_name).expect("Unable to create file");
    println!("File '{}' created successfully.", file_name);
}  