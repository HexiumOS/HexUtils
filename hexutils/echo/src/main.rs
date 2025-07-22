use sap::{Parser, Argument};

fn main() {
    let mut parser = Parser::from_env().unwrap();

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Value(val) => {
                print(&val); // Call your function
            },
            Argument::Long(opt) => println!("Error, string expected, got long option: --{}", opt),
            Argument::Short(c) => println!("Error, string expected, got short option: -{}", c),
            Argument::Stdio => println!("Error, string expected, got stdin"),
        }
    }
}

fn print(val: &str) {
    println!("{}", val);
}