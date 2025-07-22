use sap::{Argument, Parser};

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Value(val) => {
                echo(&val);
            }
            _ => {}
        }
    }
}

fn echo(val: &str) {
    println!("{}", val);
}
