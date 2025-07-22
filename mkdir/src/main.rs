use std::fs;
use std::env;

fn createdir(new_dir_path: &str) -> std::io::Result<()> {
    fs::create_dir(new_dir_path)?;
    Ok(())
}

fn createdir_parents(new_dir_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(new_dir_path)?;
    Ok(())
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].is_empty() {
        println!("No path provided.");
        return;
    } else if args.len() == 2 && args[1] == "--help" {
        println!("Usage: mkdir [ARGUMENT] [PATH]");
        println!("Usage: mkdir [PATH]");
        println!();
        println!("Creates a new directory at the specified path.");
        println!("-------------ARGUMENTS-----------------");
        println!("--parents, -p: Create parent directories as needed.");
        println!("--help: Show this help message.");
        println!("--version: Show version information.");
        return;
    } else if args.len() == 2 && args[1] == "--version" {
        println!("-------------MKDIR-RUST-----------------");
        println!("MKDIR HexiumOS/Rust rewrite version 0.1.0");
        println!();
        println!("Rewrite of the original MKDIR command, now using Rust.");
        println!("Original author: David MacKenzie");
        return;
    } else if args.len() == 3 && args[1] == "--parents" || args [1] == "-p" {
        createdir_parents(&args[2]).expect("Failed to create parent directories");
    } else {
        createdir(&args[1]).expect("Failed to create directory");
    }
}