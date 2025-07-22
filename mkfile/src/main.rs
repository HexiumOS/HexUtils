use sap::{Argument, Parser};
use std::fs::File;

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Value(val) => {
                mkfile(&val);
            }
            _ => {}
        }
    }
}

fn mkfile(file_name: &str) {
    File::create(file_name).expect("Unable to create file");
    println!("File '{}' created successfully.", file_name);
}
