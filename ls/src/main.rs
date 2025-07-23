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
    let dir = env::current_dir()?;
    visit_dirs(&dir, &|entry| {
        let path = entry.path();
        if path.is_dir() {
            println!("{}", Colour::RGB(0, 0, 255).paint(entry.file_name().to_string_lossy()));
        } else if path.is_executable() {
            println!("{}", Colour::RGB(0, 255, 0).paint(entry.file_name().to_string_lossy()));
        } else {
            println!("{}", Colour::RGB(255, 255, 255).paint(entry.file_name().to_string_lossy()));
        }
    })?;
    Ok(())
}