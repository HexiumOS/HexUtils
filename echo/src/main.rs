use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].is_empty() {
        println!("No arguments provided.");
        return;
    } else if args.len() == 2 && args[1] == "--help" {
        println!("Usage: echo [STRING]");
        println!("Prints the provided string to the standard output.");
        return;
    } else if args.len() == 2 && args[1] == "--version" {
        println!("-------------ECHO-RUST-----------------");
        println!("Echo HexiumOS/Rust rewrite version 0.1.0");
        println!();
        println!("Rewrite of the original echo command, now using Rust.");
        println!("Original authors: Brian Fox and Chet Ramey");
        return;
    } else {
        let args: Vec<String> = env::args().skip(1).collect();
        println!("{}", args.join(" "));
    }
}
