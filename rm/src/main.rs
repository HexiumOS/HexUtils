use std::env;
use std::fs;

fn main() {
        let args: Vec<String> = env::args().collect();
    if args[1].is_empty() {
        println!("No path provided.");
        return;
    } else if args.len() == 2 && args[1] == "--help" {
        println!("Usage: rm [ARGUMENT] [PATH]");
        println!("Usage: rm [PATH]");
        println!();
        println!("Removes files/directory at specified path.");
        println!("-----------------ARGUMENTS-----------------");
        println!("--recursive, -r: Remove file/directory recursively.");
        println!("--directory, -d: Remove empty directories.");
        println!("--help: Show this help message.");
        println!("--version: Show version information.");
        return;
    } else if args.len() == 2 && args[1] == "--version" {
        println!("-----------------RM-RUST-----------------");
        println!("RM HexiumOS/Rust rewrite version 0.1.0");
        println!();
        println!("Rewrite of the original RM and RMDIR, in one! Now using Rust.");
        println!("Original author of RM: Paul Rubin, David MacKenzie, Richard M. Stallman, and Jim Meyering.");
        println!("Original author of RMDIR: David MacKenzie");
        return;
    } else if args.len() == 3 && args[1] == "--recursive" || args[1] == "-r" {
        rmdir_full(&args[2]).expect("Failed to remove directory recursively");
    } else if args.len() == 3 && args[1] == "--directory" || args[1] == "-d"{
        rmdir_empty(&args[2]).expect("Failed to remove empty directory");
    } else {
        rmfile(&args[1]).expect("Failed to remove file");
    }
}

fn rmfile(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())

}

fn rmdir_empty(path: &str) -> std::io::Result<()> {
    fs::remove_dir(path)?;
    Ok(())
}

fn rmdir_full(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}