use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;
use ansi_term::Colour;
use is_executable::IsExecutable;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            cb(&entry);
        }
    }
    Ok(())
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: ls [OPTION] [PATH]");
        println!("Usage: ls [PATH]");
        println!();
        println!("Lists files and directories in the specified path.");
        println!("If no path is provided, lists files in the current directory.");
        println!("-----------------ARGUMENTS-----------------");
        println!("--all, -a: Include hidden files (those starting with a dot).");
        println!("--help: Show this help message.");
        println!("--version: Show version information.");
        return Ok(());
    } else if args.len() > 1 && args[1] == "--version" {
        println!("-----------------LS-RUST-----------------");
        println!("LS HexiumOS/Rust rewrite version 0.1.0");
        println!();
        println!("Rewrite of the original LS, in Rust.");
        println!("Original author: Richard M. Stallman and David MacKenzie.");
        return Ok(());
    } else if args.len() > 1 && (args[1] == "-a" || args[1] == "--all") {
        print!(".  ");
        print!("..  ");
        if args.len() <= 2 || args[2].is_empty() {
            let dir = env::current_dir()?;
            visit_dirs(&dir, &|entry| {
                let path = entry.path();
                if path.is_dir() {
                    print!("{}  ", Colour::RGB(0, 0, 255).paint(entry.file_name().to_string_lossy()));
                } else if path.is_executable() {
                    print!("{}  ", Colour::RGB(0, 255, 0).paint(entry.file_name().to_string_lossy()));
                } else if path.is_file() {
                    print!("{}  ", Colour::RGB(255, 255, 255).paint(entry.file_name().to_string_lossy()));
                }
            })?;
        } else {
            let path = Path::new(&args[2]);
            visit_dirs(path, &|entry| {
                let path = entry.path();
                if path.is_dir() {
                    print!("{}  ", Colour::RGB(0, 0, 255).paint(entry.file_name().to_string_lossy()));
                } else if path.is_executable() {
                    print!("{}  ", Colour::RGB(0, 255, 0).paint(entry.file_name().to_string_lossy()));
                } else if path.is_file() {
                    print!("{}  ", Colour::RGB(255, 255, 255).paint(entry.file_name().to_string_lossy()));
                }
            })?;
        }
    } else if args.len() > 1 {
        let path = Path::new(&args[1]);
        if path.exists() {
            let dir = path;
            visit_dirs(&dir, &|entry| {
                let path = entry.path();
                if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    print!("{}  ", Colour::RGB(0, 0, 255).paint(entry.file_name().to_string_lossy()));
                } else if path.is_executable() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    print!("{}  ", Colour::RGB(0, 255, 0).paint(entry.file_name().to_string_lossy()));
                } else if path.is_file() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    print!("{}  ", Colour::RGB(255, 255, 255).paint(entry.file_name().to_string_lossy()));
                }
            })?;
        }
        if !path.exists() {
            eprintln!("Error: Path '{}' does not exist.", path.display());
            return Ok(());
        }
        if !path.is_dir() {
            eprintln!("Error: '{}' is not a directory.", path.display());
            return Ok(());
        }
    } else {
        let dir = env::current_dir()?;
        visit_dirs(&dir, &|entry| {
            let path = entry.path();
            if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                print!("{}  ", Colour::RGB(0, 0, 255).paint(entry.file_name().to_string_lossy()));
            } else if path.is_executable() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                print!("{}  ", Colour::RGB(0, 255, 0).paint(entry.file_name().to_string_lossy()));
            } else if path.is_file() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                print!("{}  ", Colour::RGB(255, 255, 255).paint(entry.file_name().to_string_lossy()));
            }
        })?;
    }

    println!(); // Print a newline at the end
    Ok(())
}