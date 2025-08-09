use std::{io::Write, path::{Path, PathBuf}};

pub fn print_at_current_line(text: impl std::fmt::Display) {
    // \u1b[2k == clears current line
    print!("\u{1b}[2K\r{text}");
    std::io::stdout().flush().unwrap();
}

pub fn find_file_path(file_name: &str, path: &Path) -> Option<PathBuf> {
    print_at_current_line(format!("> Searching for {file_name} in '{}'", path.file_name().unwrap().to_str().unwrap()));
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            if let Some(buf) = find_file_path(file_name, &entry.path()) {
                return Some(buf);
            } 
        } else {
            if entry.file_name() == file_name {
                return Some(entry.path());
            }
        }
    }
    
    None
}

